use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    fmt::{self, Write},
};

use dot_ix_model::{
    common::{
        graphviz_dot_theme::GraphStyle, AnyId, DotSrcAndStyles, EdgeId, GraphvizDotTheme,
        NodeHierarchy, NodeId, TagId, TailwindClasses, TailwindKey,
    },
    info_graph::{GraphDir, InfoGraph, Tag},
    theme::{
        ColorParams, CssClassesBuilder, ElCssClasses, HighlightState, StrokeParams, Themeable,
    },
};
use indexmap::IndexMap;
use indoc::{formatdoc, writedoc};

use crate::IntoGraphvizDotSrc;

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
        let graph_attrs = graph_attrs(theme, self.direction());
        let node_attrs = node_attrs(theme);
        let edge_attrs = edge_attrs(theme);

        // Build a map from `NodeId` to their `NodeHierarchy`, so that we don't have to
        // search for it every time we want to create an edge.
        let node_id_to_hierarchy = {
            let mut node_id_to_hierarchy =
                HashMap::<&NodeId, &NodeHierarchy>::with_capacity(self.edges().len());
            let mut hierarchy_queue = VecDeque::new();
            hierarchy_queue.push_back(self.hierarchy());

            while let Some(hierarchy) = hierarchy_queue.pop_front() {
                hierarchy.iter().for_each(|(node_id, node_hierarchy)| {
                    node_id_to_hierarchy.insert(node_id, node_hierarchy);
                    hierarchy_queue.push_back(node_hierarchy);
                });
            }

            node_id_to_hierarchy
        };

        let info_graph_dot = InfoGraphDot {
            node_ids: node_id_to_hierarchy.keys().copied().collect::<Vec<_>>(),
            edge_ids: self.edges().keys().collect::<Vec<_>>(),
        };
        let el_css_classes = self.theme().el_css_classes(&info_graph_dot);

        let node_clusters = self
            .hierarchy()
            .iter()
            // Reversing the order we feed nodes to Graphviz dot tends to produce a more natural
            // layout order.
            .rev()
            .map(|(node_id, node_hierarchy)| {
                node_cluster(self, &el_css_classes, theme, node_id, node_hierarchy)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let edges = self
            .edges()
            .iter()
            .map(|(edge_id, [src_node_id, target_node_id])| {
                // We need to find the node_hierarchy for both the the `src_node_id` and
                // `target_node_id`.
                let src_node_hierarchy = node_id_to_hierarchy.get(src_node_id).copied();
                let target_node_hierarchy = node_id_to_hierarchy.get(target_node_id).copied();
                edge(
                    &el_css_classes,
                    edge_id,
                    src_node_id,
                    src_node_hierarchy,
                    target_node_id,
                    target_node_hierarchy,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut tag_legend_buffer = String::with_capacity(512 * self.tags().len() + 512);
        tag_legend(
            &mut tag_legend_buffer,
            theme,
            self.tailwind_classes(),
            self.tags(),
        )
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

        DotSrcAndStyles { dot_src, styles }
    }
}

fn graph_attrs(theme: &GraphvizDotTheme, graph_dir: GraphDir) -> String {
    let plain_text_color = theme.plain_text_color();
    // Note: `margin` is set to 0.1 because some text lies outside the viewport.
    // This may be due to incorrect width calculation for emoji characters, which
    // GraphViz falls back to the space character width.

    let node_point_size = theme.node_point_size();

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
            packmode  = "graph"
            fontcolor = "{plain_text_color}"
            fontsize  = {node_point_size}
            rankdir   = {rankdir}
        ]
        "#
    )
}

fn node_attrs(theme: &GraphvizDotTheme) -> String {
    let node_style_and_shape = match theme.graph_style {
        GraphStyle::Boxes => {
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

fn edge_attrs(theme: &GraphvizDotTheme) -> String {
    let edge_color = theme.edge_color();
    let plain_text_color = theme.plain_text_color();

    formatdoc!(
        r#"
        edge [
            arrowsize = 0.7
            color     = "{edge_color}"
            fontcolor = "{plain_text_color}"
        ]
        "#
    )
}

fn node_cluster(
    info_graph: &InfoGraph,
    el_css_classes: &ElCssClasses,
    theme: &GraphvizDotTheme,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
) -> String {
    let mut buffer = String::with_capacity(1024);

    node_cluster_internal(
        info_graph,
        el_css_classes,
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
    theme: &GraphvizDotTheme,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
    buffer: &mut String,
) -> fmt::Result {
    let node_names = info_graph.node_names();
    let node_descs = info_graph.node_descs();
    let node_emojis = info_graph.node_emojis();
    let node_tags = info_graph.node_tags();
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
    let node_label = node_name.unwrap_or(node_id);
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

    let node_tag_classes = node_tags
        .get(node_id)
        .map(|tag_ids| {
            let mut node_tag_classes = String::with_capacity(128 * tag_ids.len());
            tag_ids.iter().try_for_each(|tag_id| {
                writedoc!(
                    &mut node_tag_classes,
                    "peer-focus/{tag_id}:[&>path]:fill-lime-200 \
                    peer-focus/{tag_id}:[&>path]:stroke-lime-500"
                )
            })?;

            Ok(node_tag_classes)
        })
        .transpose()?
        .unwrap_or_else(String::new);

    if node_hierarchy.is_empty() {
        match theme.graph_style {
            GraphStyle::Boxes => writedoc!(
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
                        class = "{node_tailwind_classes} {node_tag_classes}"
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

                writedoc!(
                    buffer,
                    r#"
                        subgraph cluster_{node_id} {{
                            {node_id} [
                                label = ""
                                class = "{node_tailwind_classes} {node_tag_classes}"
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
                    class = "{node_tailwind_classes} {node_tag_classes}"
            "#
        )?;

        node_hierarchy
            .iter()
            .try_for_each(|(child_node_id, child_node_hierarchy)| {
                node_cluster_internal(
                    info_graph,
                    el_css_classes,
                    theme,
                    child_node_id,
                    child_node_hierarchy,
                    buffer,
                )
            })?;

        write!(buffer, "}}")?;
    }

    Ok(())
}

fn edge(
    el_css_classes: &ElCssClasses,
    edge_id: &EdgeId,
    src_node_id: &NodeId,
    src_node_hierarchy: Option<&NodeHierarchy>,
    target_node_id: &NodeId,
    target_node_hierarchy: Option<&NodeHierarchy>,
) -> String {
    let (edge_src_node_id, ltail) = if let Some((mut child_node_id, mut child_node_hierarchy)) =
        src_node_hierarchy
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(|node_hierarchy| node_hierarchy.last())
    {
        // This is a cluster, find the bottom / right most node.
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy.last() {
            child_node_id = next_node_id;
            child_node_hierarchy = next_node_hierarchy;
        }
        let edge_src_node_id = child_node_id;

        let ltail = Cow::Owned(format!(", ltail = cluster_{src_node_id}"));

        (edge_src_node_id, ltail)
    } else {
        // This is a node, not a cluster.
        (src_node_id, Cow::Borrowed(""))
    };

    let (edge_target_node_id, lhead) = if let Some((mut child_node_id, mut child_node_hierarchy)) =
        target_node_hierarchy
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(|node_hierarchy| node_hierarchy.first())
    {
        // This is a cluster, find the top / left most node.
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy.first() {
            child_node_id = next_node_id;
            child_node_hierarchy = next_node_hierarchy;
        }
        let edge_target_node_id = child_node_id;

        let lhead = Cow::Owned(format!(", lhead = cluster_{target_node_id}"));

        (edge_target_node_id, lhead)
    } else {
        // This is a node, not a cluster.
        (target_node_id, Cow::Borrowed(""))
    };

    let edge_css_classes = el_css_classes
        .get(&AnyId::from(edge_id.clone()))
        .map(AsRef::<str>::as_ref)
        .map(|edge_css_classes| format!(", class = \"{edge_css_classes}\""));
    let edge_css_classes = edge_css_classes.as_deref().unwrap_or_default();

    formatdoc!(
        r#"
        {edge_src_node_id} -> {edge_target_node_id} [
            id     = "{edge_id}",
            minlen = 3
            {edge_css_classes}
            {ltail}
            {lhead}
        ]"#
    )
}

fn tag_legend(
    buffer: &mut String,
    theme: &GraphvizDotTheme,
    tailwind_classes: &TailwindClasses,
    tags: &IndexMap<TagId, Tag>,
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
    tags.iter().try_for_each(|(tag_id, tag)| {
        let tag_label = tag.name(); // TODO: escape

        // This is for tailwindcss to identify this peer by name.
        let tag_peer_class = format!("peer/{tag_id}");

        let tag_classes = tailwind_classes
            .get(&TailwindKey::AnyId(tag_id.clone().into()))
            .map(String::as_str)
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
                class     = "{tag_classes} {tag_peer_class}"
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

#[derive(Clone)]
struct InfoGraphDot<'graph> {
    node_ids: Vec<&'graph NodeId>,
    edge_ids: Vec<&'graph EdgeId>,
}

impl<'graph> Themeable for InfoGraphDot<'graph> {
    fn node_ids(&self) -> impl Iterator<Item = &NodeId>
    where
        Self: Sized,
    {
        self.node_ids.iter().copied()
    }

    fn node_outline_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        path_color_classes(builder, color_params, "outline");
        outline_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "[&>path]:",
        );
    }

    fn node_stroke_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        path_color_classes(builder, color_params, "stroke");
        border_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
        );
    }

    fn node_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        path_color_classes(builder, color_params, "fill");
    }

    fn edge_ids(&self) -> impl Iterator<Item = &EdgeId>
    where
        Self: Sized,
    {
        self.edge_ids.iter().copied()
    }

    fn edge_outline_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        el_color_classes(builder, color_params, "outline", "");
        outline_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "",
        );
    }

    fn edge_stroke_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        path_color_classes(builder, color_params, "stroke");
        polygon_color_classes(builder, color_params, "stroke");
        border_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
        );
    }

    fn edge_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        // deliberately don't have `"fill"` for `path`, because that adds a thin line
        // when the path is styled as dashed
        polygon_color_classes(builder, color_params, "fill");
    }
}

fn outline_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    stroke_width: &str,
    stroke_style: &str,
    el_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    builder
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{stroke_width}"
        ))
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{stroke_style}"
        ));
}

/// Appends SVG stroke classes that emulate HTML border styles.
fn border_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    stroke_width: &str,
    stroke_style: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    match stroke_style {
        "none" => {}
        "solid" => {
            builder.append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"));
        }
        "dashed" => {
            builder
                .append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"))
                .append(&format!("[&>path]:{highlight_prefix}[stroke-dasharray:3]"));
        }
        "dotted" => {
            builder
                .append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"))
                .append(&format!("[&>path]:{highlight_prefix}[stroke-dasharray:2]"));
        }
        _ => {}
    };
}

fn path_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
) {
    el_color_classes(builder, color_params, stroke_or_fill, "[&>path]:")
}

fn polygon_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
) {
    el_color_classes(builder, color_params, stroke_or_fill, "[&>polygon]:")
}

fn el_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
    el_prefix: &str,
) {
    let ColorParams {
        highlight_state,
        color,
        shade,
    } = color_params;

    let highlight_prefix = highlight_prefix(highlight_state);
    builder.append(&format!(
        "{el_prefix}{highlight_prefix}{stroke_or_fill}-{color}-{shade}"
    ));
}

fn highlight_prefix(highlight_state: HighlightState) -> &'static str {
    match highlight_state {
        HighlightState::Normal => "",
        HighlightState::Focus => "focus:",
        HighlightState::FocusHover => "focus:hover:",
        HighlightState::Hover => "hover:",
        HighlightState::Active => "focus:active:",
    }
}
