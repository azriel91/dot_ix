use std::{
    borrow::Cow,
    fmt::{self, Write},
};

use dot_ix_model::{
    common::{
        graphviz_attrs::EdgeDir, AnyId, DotSrcAndStyles, EdgeId, GraphvizAttrs, GraphvizDotTheme,
        NodeHierarchy, NodeId, TagId, TagNames,
    },
    info_graph::{GraphDir, GraphStyle, InfoGraph},
    theme::{ElCssClasses, Theme},
};
use indexmap::{IndexMap, IndexSet};
use indoc::{formatdoc, writedoc};

use crate::{InfoGraphDot, IntoGraphvizDotSrc};

/// Hack to get Chrome/Edge to not display black box around focused nodes.
const OUTLINE_NONE: &str = "outline-none";

/// Renders a GraphViz Dot diagram with interactive styling.
///
/// This is currently a mashed together implementation. A proper implementation
/// would require investigation into:
///
/// * Whether colours and border sizes should be part of a theme object that we
///   take in.
/// * Whether colours and border sizes can be configured through CSS classes,
///   and within the generated dot source, we only apply those classes.
///
/// # Notes
///
/// ## GraphViz Version and Errors
///
/// Many package managers ship Graphviz 2.43, which is a really old version.
/// In 2.43.0, when cluster labels are too short, and there are edges between
/// clusters, `dot` fails with:
///
/// ```text
/// dot: compound.c:452: makeCompoundEdge: Assertion `bez->sflag' failed.
/// ```
///
/// See [graphviz#1879] and [graphviz#1949].
/// This can be avoided by handling requests using an up-to-date version of
/// GraphViz -- 8.1.0 at the time of writing.
///
/// [graphviz#1879]: https://gitlab.com/graphviz/graphviz/-/issues/1879
/// [graphviz#1949]: https://gitlab.com/graphviz/graphviz/-/issues/1949
///
/// ## Font Metrics
///
/// Internally, GraphViz hardcodes font metrics for width calculation.
///
/// The exhaustive list of fonts for version `8.1.0`, from
/// `lib/common/textspan_lut.c` -- search for `(const char *[])`, are:
///
/// ```text
/// "Calibri", "Georgia", "Nunito", "OpenSans", "Trebuchet MS", "Trebuchet",
/// "Verdana", "albany", "arial", "arialmt", "arimo", "consola", "consolas",
/// "cour", "courier", "couriernew", "cousine", "cumberland", "dejavusans",
/// "freemono", "freesans", "freeserif", "helvetica", "liberationmono",
/// "liberationsans", "liberationserif", "nimbusmono", "nimbusroman",
/// "nimbussans", "nimbussansa", "texgyrecursor", "texgyreheros",
/// "texgyretermes", "thorndale", "times", "timesnewroman", "timesroman",
/// "tinos"
/// ```
///
/// However, not all of these fonts are available to end users, and so web
/// application developers must ship the fonts to the users, and also take into
/// account licensing.
///
/// See [`liberationmono`] for a monospace font -- the download from the Webfont
/// Kit tab includes instruction for setting this up.
///
/// [`liberationmono`]: https://www.fontsquirrel.com/fonts/liberation-mono
///
/// ## Tailwind CSS Generation
///
/// Currently tailwind compilation is done on `.rs` source. This means the
/// classes must be fully spelt out in source, and cannot be computed string
/// formats.
///
/// If we were to support runtime tailwind CSS generation, e.g. the user
/// specifying a colour which is plugged into a `format!("fill-{colour}")`, then
/// we need to run tailwind on the generated dot source.
///
/// The most complete crate for this at the tim of writing is [`tailwind-css`],
/// however it uses the MPL-2.0 license which is copyleft, requiring a binary
/// compiled with it to be open source, which is not something users of `dot_ix`
/// may be able to work with.
///
/// [`tailwind-css`]: https://github.com/oovm/tailwind-rs
impl IntoGraphvizDotSrc for &InfoGraph {
    fn into(self, theme: &GraphvizDotTheme) -> DotSrcAndStyles {
        let graph_style = self.graph_style();
        let graphviz_attrs = self.graphviz_attrs();
        let graph_attrs = graph_attrs(theme, self.direction(), graphviz_attrs);
        let node_attrs = node_attrs(graph_style, theme);
        let edge_attrs = edge_attrs(graphviz_attrs, theme);
        let diagram_theme = self.theme();

        // Build a map from `NodeId` to their `NodeHierarchy`, so that we don't have to
        // search for it every time we want to create an edge.
        let node_id_to_hierarchy = self.hierarchy_flat();
        let node_id_to_hierarchy = &node_id_to_hierarchy;

        let info_graph_dot = InfoGraphDot {
            graph_style,
            node_id_to_hierarchy,
            node_ids: node_id_to_hierarchy.keys().copied().collect::<Vec<_>>(),
            edge_ids: self.edges().keys().collect::<Vec<_>>(),
        };
        let info_graph_dot = &info_graph_dot;
        let (el_css_classes, diagram_theme_warnings) = diagram_theme.el_css_classes(info_graph_dot);
        let el_css_classes = &el_css_classes;

        // tag styles per tag
        let tag_styles_focus = self.tag_styles_focus();
        let (tag_el_css_classes_map, theme_warnings) = self.tags().keys().fold(
            (
                IndexMap::<&TagId, ElCssClasses>::new(),
                diagram_theme_warnings,
            ),
            |(mut tag_el_css_classes_map_acc, mut theme_warnings_acc), tag_id| {
                let tag_theme = tag_styles_focus
                    .get(tag_id)
                    .cloned()
                    .map(Theme::from)
                    .unwrap_or_else(Theme::tag_base);
                let (tag_el_css_classes, tag_theme_warnings) =
                    tag_theme.tag_el_css_classes(info_graph_dot, diagram_theme, tag_id);

                tag_el_css_classes_map_acc.insert(tag_id, tag_el_css_classes);
                theme_warnings_acc.extend(tag_theme_warnings.into_inner());

                (tag_el_css_classes_map_acc, theme_warnings_acc)
            },
        );
        let tag_el_css_classes_map = &tag_el_css_classes_map;

        let node_clusters = self
            .hierarchy()
            .iter()
            // Reversing the order we feed nodes to Graphviz dot tends to produce a more natural
            // layout order.
            .rev()
            .map(|(node_id, node_hierarchy)| {
                node_cluster(
                    self,
                    el_css_classes,
                    tag_el_css_classes_map,
                    theme,
                    node_id,
                    node_hierarchy,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let edge_tags_set = self.edge_tags_set();
        let edge_tags_set = &edge_tags_set;
        let edges = self
            .edges()
            .iter()
            .map(|(edge_id, [src_node_id, target_node_id])| {
                let edge_desc = self.edge_descs().get(edge_id).map(String::as_str);
                let edge_constraint = graphviz_attrs.edge_constraints().get(edge_id).copied();
                let edge_dir = graphviz_attrs.edge_dirs().get(edge_id).copied();
                let edge_minlen = graphviz_attrs.edge_minlens().get(edge_id).copied();
                let edge_tags = edge_tags_set.get(edge_id);

                // Graphviz has a bug where setting the `headport` / `tailport` attributes
                // causes the edge to not be rendered with spline curves if the `lhead` /
                // `ltail` is a node. So we workaround this by still passing the
                // compass points through as the source / target node IDs.
                let (src_node_id_plain, src_compass_point) = match src_node_id.split_once(':') {
                    Some((src_node_id_plain, src_compass_point)) => {
                        (src_node_id_plain, Some(src_compass_point))
                    }
                    None => (src_node_id.as_str(), None),
                };
                let (target_node_id_plain, target_compass_point) =
                    match target_node_id.split_once(':') {
                        Some((target_node_id_plain, target_compass_point)) => {
                            (target_node_id_plain, Some(target_compass_point))
                        }
                        None => (target_node_id.as_str(), None),
                    };

                // We need to find the node_hierarchy for both the the `src_node_id` and
                // `target_node_id`.
                let src_node_hierarchy = node_id_to_hierarchy.get(src_node_id_plain).copied();
                let target_node_hierarchy = node_id_to_hierarchy.get(target_node_id_plain).copied();

                let edge_args = EdgeArgs {
                    el_css_classes,
                    tag_el_css_classes_map,
                    edge_tags,
                    edge_id,
                    edge_desc,
                    edge_constraint,
                    edge_dir,
                    edge_minlen,
                    src_node_id_with_port: src_node_id.as_str(),
                    src_node_id_plain,
                    src_compass_point,
                    src_node_hierarchy,
                    target_node_id_with_port: target_node_id.as_str(),
                    target_node_id_plain,
                    target_node_hierarchy,
                    target_compass_point,
                };

                edge(edge_args)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut tag_legend_buffer = String::with_capacity(512 * self.tags().len() + 512);
        tag_legend(&mut tag_legend_buffer, theme, el_css_classes, self.tags())
            .expect("Failed to write `tag_legend` string.");

        let dot_src = formatdoc!(
            "digraph G {{
                {graph_attrs}
                {node_attrs}
                {edge_attrs}

                {tag_legend_buffer}

                {node_clusters}

                {edges}
            }}"
        );

        let styles = self.css().to_string();

        DotSrcAndStyles {
            dot_src,
            styles,
            theme_warnings,
        }
    }
}

fn graph_attrs(
    theme: &GraphvizDotTheme,
    graph_dir: GraphDir,
    graphviz_attrs: &GraphvizAttrs,
) -> String {
    let plain_text_color = theme.plain_text_color();
    // Note: `margin` is set to 0.1 because some text lies outside the viewport.
    // This may be due to incorrect width calculation for emoji characters, which
    // GraphViz falls back to the space character width.

    let node_point_size = theme.node_point_size();

    let pack_mode = &graphviz_attrs.pack_mode;

    let rankdir = match graph_dir {
        GraphDir::Horizontal => "LR",
        GraphDir::Vertical => "TB",
    };

    formatdoc!(
        r#"
        compound  = true
        graph [
            margin    = 0.1
            penwidth  = 0
            nodesep   = 0.0
            ranksep   = 0.02
            bgcolor   = "transparent"
            fontname  = "helvetica"
            packmode  = "{pack_mode}"
            fontcolor = "{plain_text_color}"
            fontsize  = {node_point_size}
            rankdir   = {rankdir}
        ]
        "#
    )
}

fn node_attrs(graph_style: GraphStyle, theme: &GraphvizDotTheme) -> String {
    let node_style_and_shape = match graph_style {
        GraphStyle::Box => {
            "shape     = \"rect\"
            style     = \"rounded,filled\""
        }
        GraphStyle::Circle => {
            "shape    = \"circle\"
            style     = \"filled\""
        }
    };
    let node_text_color = theme.node_text_color();
    let node_point_size = theme.node_point_size();
    let node_width = theme.node_width();
    let node_height = theme.node_height();
    let node_margin_x = theme.node_margin_x();
    let node_margin_y = theme.node_margin_y();

    formatdoc!(
        r#"
        node [
            fontcolor = "{node_text_color}"
            fontname  = "liberationmono"
            fontsize  = {node_point_size}
            {node_style_and_shape}
            width     = {node_width}
            height    = {node_height}
            margin    = "{node_margin_x:.3},{node_margin_y:.3}"
        ]
        "#
    )
}

fn edge_attrs(graphviz_attrs: &GraphvizAttrs, theme: &GraphvizDotTheme) -> String {
    let edge_color = theme.edge_color();
    let plain_text_color = theme.plain_text_color();
    let edge_point_size = theme.edge_point_size();
    let edge_constraint_default = graphviz_attrs.edge_constraint_default();
    let edge_dir_default = graphviz_attrs.edge_dir_default();
    let edge_minlen_default = graphviz_attrs.edge_minlen_default();

    formatdoc!(
        r#"
        edge [
            constraint = {edge_constraint_default},
            dir        = {edge_dir_default},
            minlen     = {edge_minlen_default},
            fontname   = "liberationmono"
            fontsize   = {edge_point_size}
            arrowsize  = 0.7
            color      = "{edge_color}"
            fontcolor  = "{plain_text_color}"
        ]
        "#
    )
}

fn node_cluster(
    info_graph: &InfoGraph,
    el_css_classes: &ElCssClasses,
    tag_el_css_classes_map: &IndexMap<&TagId, ElCssClasses>,
    theme: &GraphvizDotTheme,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
) -> String {
    let mut buffer = String::with_capacity(1024);

    node_cluster_internal(
        info_graph,
        el_css_classes,
        tag_el_css_classes_map,
        theme,
        node_id,
        node_hierarchy,
        &mut buffer,
    )
    .expect("Failed to write node_cluster string.");

    buffer
}

fn node_cluster_internal(
    info_graph: &InfoGraph,
    el_css_classes: &ElCssClasses,
    tag_el_css_classes_map: &IndexMap<&TagId, ElCssClasses>,
    theme: &GraphvizDotTheme,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
    buffer: &mut String,
) -> fmt::Result {
    let graph_style = info_graph.graph_style();
    let node_names = info_graph.node_names();
    let node_descs = info_graph.node_descs();
    let node_emojis = info_graph.node_emojis();
    let node_tags_set = info_graph.node_tags_set();
    let graph_dir = info_graph.direction();
    let node_tailwind_classes = el_css_classes
        .get(&AnyId::from(node_id.clone()))
        .map(AsRef::<str>::as_ref)
        .unwrap_or_default();

    let node_point_size = theme.node_point_size();
    let node_name = node_names.get(node_id).map(String::as_str);
    let node_desc = node_descs.get(node_id).map(String::as_str);
    let node_emoji = node_emojis.get(node_id).map(String::as_str);
    // TODO: escape
    let node_label = node_name.unwrap_or(node_id).replace(' ', "&nbsp;");
    // TODO: escape
    let node_desc = node_desc
        .map(|desc| desc.replace('\n', "<br />"))
        .map(|desc| format!("<tr><td balign=\"left\">{desc}</td></tr>"));

    let emoji = node_emoji.map(|emoji| {
        let emoji_rowspan = if node_desc.is_some() {
            "rowspan=\"2\""
        } else {
            ""
        };

        // Graphviz uses one space character per byte in the emoji.
        //
        // Because emojis tend to be 4 bytes long, the width of the cell tends to be 4
        // times what it should be.
        //
        // Specifying the following attributes, plus the nested table, is a hardcoded
        // hack to fix that:
        //
        // * `align`
        // * `balign`
        // * `fixedsized`
        // * `width`
        // * `height`
        let cell_spacing = 2;
        let emoji_point_size = theme.emoji_point_size();
        let emoji_point_size_spaced = emoji_point_size + cell_spacing;
        let row_height = if node_desc.is_some() {
            node_point_size * 2
        } else {
            node_point_size
        };
        formatdoc!(
            "\
            <td \
                valign=\"top\" \
                {emoji_rowspan}
            >\
                <table \
                    border=\"0\" \
                    cellborder=\"0\" \
                    cellpadding=\"0\" \
                    cellspacing=\"{cell_spacing}\" \
                >\
                    <tr><td \
                        fixedsize=\"true\" \
                        width=\"{emoji_point_size_spaced}\" \
                        height=\"{row_height}\" \
                        align=\"left\" \
                        balign=\"left\" \
                    >\
                        <font point-size=\"{emoji_point_size}\">{emoji}</font>\
                    </td></tr>
                </table>
            </td>"
        )
    });
    let node_desc = node_desc.as_deref().unwrap_or("");
    let emoji = emoji.as_deref().unwrap_or("");

    let node_tag_classes = node_tags_set
        .get(node_id)
        .map(|tag_ids| {
            tag_ids
                .iter()
                .filter_map(|tag_id| tag_el_css_classes_map.get(tag_id))
                .map(|el_css_classes| {
                    el_css_classes
                        .get(&AnyId::from(node_id.clone()))
                        .map(AsRef::<str>::as_ref)
                        .unwrap_or_default()
                })
                .collect::<String>()
        })
        .unwrap_or_default();

    // Note: There's no space between `{node_tailwind_classes}{node_tag_classes}`
    // because for some reason spaces before `{node_tag_classes}` are translated
    // into the `0xa0` byte.
    //
    // Same thing happens for `{edge_tag_classes}`
    if node_hierarchy.is_empty() {
        match graph_style {
            GraphStyle::Box => writedoc!(
                buffer,
                r#"
                    {node_id} [
                        label = <<table
                            border="0"
                            cellborder="0"
                            cellpadding="0"
                            cellspacing="0"
                        >
                            <tr>
                                {emoji} <td align="left" balign="left">{node_label}</td>
                            </tr>
                            {node_desc}
                        </table>>
                        class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
                    ]
                "#
            )?,
            GraphStyle::Circle => {
                // `margin` doesn't apply to `plain` shaped nodes, so we use rectangle and use
                // an invisible colour.
                let margin = match graph_dir {
                    GraphDir::Horizontal => "margin = \"0.11,0.07\"",
                    GraphDir::Vertical => "margin = \"0.13,0.055\"",
                };

                let no_color = "#00000000";

                // We need to reset the `label`, `margin`, and `class` attributes for the
                // internal cluster, otherwise it inherits from the parent cluster.
                writedoc!(
                    buffer,
                    r#"
                        subgraph cluster_{node_id} {{
                            label = <>
                            margin = 0.0
                            class = "{OUTLINE_NONE}"

                            {node_id} [
                                label = ""
                                class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
                                {margin}
                            ]
                            {node_id}_text [
                                fillcolor="{no_color}"
                                shape="rectangle"
                                {margin}
                                label = <<table
                                    border="0"
                                    cellborder="0"
                                    cellpadding="0"
                                    cellspacing="0"
                                >
                                    <tr>
                                        {emoji} <td align="left" balign="left">{node_label}</td>
                                    </tr>
                                    {node_desc}
                                </table>>
                            ]
                        }}
                    "#
                )?
            }
        }
    } else {
        writedoc!(
            buffer,
            r#"
                subgraph cluster_{node_id} {{
                    margin = {node_point_size}
                    label = <<table
                        border="0"
                        cellborder="0"
                        cellpadding="0"
                        cellspacing="0"
                    >
                        <tr>
                            {emoji} <td align="left" balign="left">{node_label}</td>
                        </tr>
                        {node_desc}
                    </table>>
                    style = "filled,rounded"
                    class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
            "#
        )?;

        match graph_dir {
            GraphDir::Horizontal => node_hierarchy
                .iter()
                // Reversing the order we feed nodes to Graphviz dot tends to produce a more natural
                // layout order.
                .rev()
                .try_for_each(|(child_node_id, child_node_hierarchy)| {
                    node_cluster_internal(
                        info_graph,
                        el_css_classes,
                        tag_el_css_classes_map,
                        theme,
                        child_node_id,
                        child_node_hierarchy,
                        buffer,
                    )
                })?,
            GraphDir::Vertical => {
                node_hierarchy
                    .iter()
                    .try_for_each(|(child_node_id, child_node_hierarchy)| {
                        node_cluster_internal(
                            info_graph,
                            el_css_classes,
                            tag_el_css_classes_map,
                            theme,
                            child_node_id,
                            child_node_hierarchy,
                            buffer,
                        )
                    })?
            }
        }

        write!(buffer, "}}")?;
    }

    Ok(())
}

struct EdgeArgs<'args> {
    el_css_classes: &'args ElCssClasses,
    tag_el_css_classes_map: &'args IndexMap<&'args TagId, ElCssClasses>,
    edge_id: &'args EdgeId,
    edge_desc: Option<&'args str>,
    edge_constraint: Option<bool>,
    edge_dir: Option<EdgeDir>,
    edge_minlen: Option<u32>,
    edge_tags: Option<&'args IndexSet<TagId>>,
    src_node_id_with_port: &'args str,
    src_node_id_plain: &'args str,
    src_node_hierarchy: Option<&'args NodeHierarchy>,
    src_compass_point: Option<&'args str>,
    target_node_id_with_port: &'args str,
    target_node_id_plain: &'args str,
    target_node_hierarchy: Option<&'args NodeHierarchy>,
    target_compass_point: Option<&'args str>,
}

fn edge(edge_args: EdgeArgs<'_>) -> String {
    let EdgeArgs {
        el_css_classes,
        tag_el_css_classes_map,
        edge_id,
        edge_desc,
        edge_constraint,
        edge_dir,
        edge_minlen,
        edge_tags,
        src_node_id_with_port,
        src_node_id_plain,
        src_node_hierarchy,
        src_compass_point,
        target_node_id_with_port,
        target_node_id_plain,
        target_node_hierarchy,
        target_compass_point,
    } = edge_args;
    let (edge_src_node_id, ltail) = if let Some((mut child_node_id, mut child_node_hierarchy)) =
        src_node_hierarchy
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(middle_node)
    {
        // This is a cluster, find the innermost node.
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy.last() {
            child_node_id = next_node_id;
            child_node_hierarchy = next_node_hierarchy;
        }
        let edge_src_node_id = child_node_id;

        let mut ltail = format!(", ltail = cluster_{src_node_id_plain}");
        if let Some(src_compass_point) = src_compass_point {
            ltail.push_str(" tailport = ");
            ltail.push_str(src_compass_point);
        }

        (edge_src_node_id.as_str(), Cow::Owned(ltail))
    } else {
        // This is a node, not a cluster.
        (src_node_id_with_port, Cow::Borrowed(""))
    };

    let (edge_target_node_id, lhead) = if let Some((mut child_node_id, mut child_node_hierarchy)) =
        target_node_hierarchy
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(middle_node)
    {
        // This is a cluster, find the innermost node.
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy.first() {
            child_node_id = next_node_id;
            child_node_hierarchy = next_node_hierarchy;
        }
        let edge_target_node_id = child_node_id;

        let mut lhead = format!(", lhead = cluster_{target_node_id_plain}");
        if let Some(target_compass_point) = target_compass_point {
            lhead.push_str(" headport = ");
            lhead.push_str(target_compass_point);
        }

        (edge_target_node_id.as_str(), Cow::Owned(lhead))
    } else {
        // This is a node, not a cluster.
        (target_node_id_with_port, Cow::Borrowed(""))
    };

    let edge_label = edge_desc
        .map(|edge_desc| Cow::Owned(format!("label = <{edge_desc}>")))
        .unwrap_or(Cow::Borrowed(""));
    let edge_tailwind_classes = el_css_classes
        .get(&AnyId::from(edge_id.clone()))
        .map(AsRef::<str>::as_ref)
        .unwrap_or_default();

    let edge_tag_classes = edge_tags
        .map(|tag_ids| {
            tag_ids
                .iter()
                .filter_map(|tag_id| tag_el_css_classes_map.get(tag_id))
                .map(|el_css_classes| {
                    el_css_classes
                        .get(&AnyId::from(edge_id.clone()))
                        .map(AsRef::<str>::as_ref)
                        .unwrap_or_default()
                })
                .collect::<String>()
        })
        .unwrap_or_default();
    let edge_constraint = edge_constraint
        .map(|edge_constraint| Cow::Owned(format!("constraint = {edge_constraint}")))
        .unwrap_or(Cow::Borrowed(""));
    let edge_dir = edge_dir
        .map(|edge_dir| Cow::Owned(format!("dir = {edge_dir}")))
        .unwrap_or(Cow::Borrowed(""));
    let edge_minlen = edge_minlen
        .map(|edge_minlen| Cow::Owned(format!("minlen = {edge_minlen}")))
        .unwrap_or(Cow::Borrowed(""));

    // Note: There's no space between `{edge_tailwind_classes}{edge_tag_classes}`
    // because for some reason spaces before `{edge_tag_classes}` are translated
    // into the `0xa0` byte.
    //
    // Same thing happens for `{node_tag_classes}`
    formatdoc!(
        r#"
        {edge_src_node_id} -> {edge_target_node_id} [
            id     = "{edge_id}"
            {edge_label}
            {edge_constraint}
            {edge_dir}
            {edge_minlen}
            class = "{edge_tailwind_classes}{edge_tag_classes}"
            {ltail}
            {lhead}
        ]"#
    )
}

/// Returns the middle node and its hierarchy for a given cluster.
fn middle_node(node_hierarchy: &NodeHierarchy) -> Option<(&NodeId, &NodeHierarchy)> {
    let half_index = node_hierarchy.len() / 2;

    // I'm not sure why we subtract 1 instead of add 1 to get the better
    // `half_index`, but by adding 1, the edge shifted closer to one side of the
    // cluster instead of the middle.
    let node_index = if half_index == 0 {
        half_index
    } else {
        half_index - 1
    };
    node_hierarchy.get_index(node_index)
}

fn tag_legend(
    buffer: &mut String,
    theme: &GraphvizDotTheme,
    el_css_classes: &ElCssClasses,
    tags: &TagNames,
) -> fmt::Result {
    let node_point_size = theme.node_point_size();
    writedoc!(
        buffer,
        "subgraph cluster_tag_legend {{
            margin = {node_point_size}
            label = <<b>Legend</b>>
            style = rounded
        "
    )?;

    let tag_width = theme.tag_width();
    let tag_height = theme.tag_height();
    let tag_margin_x = theme.tag_margin_x();
    let tag_margin_y = theme.tag_margin_y();
    let tag_point_size = theme.tag_point_size();
    let tag_classes = theme.tag_classes().trim();
    tags.iter().try_for_each(|(tag_id, tag_name)| {
        let tag_label = tag_name; // TODO: escape

        // This is for tailwindcss to identify this peer by name.
        let tag_peer_class = format!("peer/{tag_id}");

        let tag_classes = el_css_classes
            .get(&AnyId::from(tag_id.clone()))
            .map(AsRef::<str>::as_ref)
            .unwrap_or(tag_classes);

        writedoc!(
            buffer,
            r#"
            subgraph cluster_{tag_id} {{
                label     = <{tag_label}>
                width     = {tag_width}
                height    = {tag_height}
                margin    = "{tag_margin_x:.3},{tag_margin_y:.3}"
                fontname  = "liberationmono"
                fontsize  = {tag_point_size}
                class     = "{OUTLINE_NONE} {tag_classes} {tag_peer_class}"
                penwidth  = 1

                // invisible node for cluster to appear
                {tag_id} [
                    fixedsize = true
                    width     = 0.01
                    height    = 0.01
                    margin    = "0.0,0.0"
                    shape     = point
                ]
            }}
            "#
        )?;

        Ok(())
    })?;

    // Add invisible edge between tags to enforce ordering
    let mut tag_ids_iter = tags.keys();
    if let Some(mut tag_id_current) = tag_ids_iter.next() {
        for tag_id_next in tag_ids_iter {
            writeln!(
                buffer,
                "    {tag_id_current} -> {tag_id_next} [style = invis]"
            )?;
            tag_id_current = tag_id_next;
        }
    }

    writeln!(buffer, "}}")?;

    Ok(())
}
