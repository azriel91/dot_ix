use std::{
    borrow::Cow,
    fmt::{self, Write},
};

use dot_ix_model::{
    common::{
        dot_src_and_styles::{GraphvizImage, GraphvizOpts},
        graphviz_attrs::{EdgeDir, FixedSize, NodeHeights, NodeWidths, Splines},
        AnyId, DotSrcAndStyles, EdgeId, GraphvizAttrs, GraphvizDotTheme, ImageId, Images,
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
        let node_attrs = node_attrs(graph_style, graphviz_attrs, theme);
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

        let node_widths = graphviz_attrs.node_widths();
        let node_heights = graphviz_attrs.node_heights();
        let node_clusters = self
            .hierarchy()
            .iter()
            .map(|(node_id, node_hierarchy)| {
                let node_cluster_args = NodeClusterArgs {
                    info_graph: self,
                    el_css_classes,
                    tag_el_css_classes_map,
                    theme,
                    node_id,
                    node_hierarchy,
                    node_widths,
                    node_heights,
                };

                node_cluster(node_cluster_args)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let edges = edges(
            self,
            node_id_to_hierarchy,
            el_css_classes,
            tag_el_css_classes_map,
        );

        let mut tag_legend_buffer = String::with_capacity(512 * self.tags().len() + 512);
        tag_legend(
            self.direction(),
            &mut tag_legend_buffer,
            theme,
            el_css_classes,
            self.tags(),
        )
        .expect("Failed to write `tag_legend` string.");

        let opts = {
            let images = self
                .images()
                .iter()
                .map(|(image_id, image)| GraphvizImage {
                    path: image_id.as_str().to_string(),
                    width: image.width().to_string(),
                    height: image.height().to_string(),
                })
                .collect();

            GraphvizOpts::new(images)
        };

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
            opts,
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

    let nodesep = graphviz_attrs.nodesep();
    let ranksep = graphviz_attrs.ranksep();
    let splines = graphviz_attrs.splines();
    let splines = match splines {
        Splines::Unset => Cow::Borrowed(""),
        Splines::None
        | Splines::Line
        | Splines::Polyline
        | Splines::Curved
        | Splines::Ortho
        | Splines::Spline => Cow::Owned(format!("splines = {splines}")),
    };

    formatdoc!(
        r#"
        compound  = true
        graph [
            margin    = 0.1
            nodesep   = {nodesep}
            ranksep   = {ranksep}
            bgcolor   = "transparent"
            fontname  = "helvetica"
            packmode  = "{pack_mode}"
            fontcolor = "{plain_text_color}"
            fontsize  = {node_point_size}
            rankdir   = {rankdir}
            {splines}
        ]
        "#
    )
}

fn node_attrs(
    graph_style: GraphStyle,
    graphviz_attrs: &GraphvizAttrs,
    theme: &GraphvizDotTheme,
) -> String {
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

    let node_margin = graphviz_attrs.margin_node_default();
    let node_width = graphviz_attrs.node_width_default();
    let node_height = graphviz_attrs.node_height_default();
    let fixed_size = graphviz_attrs.fixed_size();
    let fixed_size = match fixed_size {
        FixedSize::False => Cow::Borrowed(""),
        FixedSize::True | FixedSize::Shape => Cow::Owned(format!(r#"fixedsize = {fixed_size}"#)),
    };

    formatdoc!(
        r#"
        node [
            fontcolor = "{node_text_color}"
            fontname  = "liberationmono"
            fontsize  = {node_point_size}
            {node_style_and_shape}
            width     = {node_width}
            height    = {node_height}
            margin    = "{node_margin}"
            {fixed_size}
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

fn node_cluster(node_cluster_args: NodeClusterArgs<'_>) -> String {
    let mut buffer = String::with_capacity(1024);

    node_cluster_internal(node_cluster_args, &mut buffer)
        .expect("Failed to write node_cluster string.");

    buffer
}

fn node_cluster_internal(
    node_cluster_args: NodeClusterArgs<'_>,
    buffer: &mut String,
) -> fmt::Result {
    let NodeClusterArgs {
        info_graph,
        el_css_classes,
        tag_el_css_classes_map,
        theme,
        node_id,
        node_hierarchy,
        node_widths,
        node_heights,
    } = node_cluster_args;

    let graph_style = info_graph.graph_style();
    let node_names = info_graph.node_names();
    let node_descs = info_graph.node_descs();
    let node_emojis = info_graph.node_emojis();
    let node_tags_set = info_graph.node_tags_set();
    let images = info_graph.images();
    let node_images = info_graph.node_images();
    let graph_dir = info_graph.direction();
    let node_tailwind_classes = el_css_classes
        .get(&AnyId::from(node_id.clone()))
        .map(AsRef::<str>::as_ref)
        .unwrap_or_default();
    let graphviz_attrs = info_graph.graphviz_attrs();
    let margins = graphviz_attrs.margins();

    let node_point_size = theme.node_point_size();
    let node_name = node_names.get(node_id).map(String::as_str);
    let node_desc = node_descs.get(node_id).map(String::as_str);
    let node_emoji = node_emojis.get(node_id).map(String::as_str);
    let node_image = node_images.get(node_id);
    // TODO: escape
    let node_label = node_name.unwrap_or(node_id).replace(' ', "&nbsp;");
    // TODO: escape
    let node_desc = node_desc
        .map(|desc| desc.replace('\n', "<br />"))
        .map(|desc| format!("<tr><td balign=\"left\">{desc}</td></tr>"));
    let node_desc = node_desc.as_deref();

    let image = image(images, node_image, node_desc);
    let image = image.as_deref().unwrap_or("");
    let emoji = emoji(node_emoji, node_desc, theme, node_point_size);
    let emoji = emoji.as_deref().unwrap_or("");
    let node_desc = node_desc.unwrap_or("");

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

    let node_width = node_widths
        .get(node_id)
        .map(|node_width| Cow::<str>::Owned(format!("width = {node_width}")))
        .unwrap_or_default();
    let node_height = node_heights
        .get(node_id)
        .map(|node_height| Cow::<str>::Owned(format!("height = {node_height}")))
        .unwrap_or_default();

    // Note: There's no space between `{node_tailwind_classes}{node_tag_classes}`
    // because for some reason spaces before `{node_tag_classes}` are translated
    // into the `0xa0` byte.
    //
    // Same thing happens for `{edge_tag_classes}`
    if node_hierarchy.is_empty() {
        match graph_style {
            GraphStyle::Box => {
                let margin = margins
                    .get(node_id)
                    .copied()
                    .map(|margin| Cow::<str>::Owned(format!(r#"margin = "{margin}""#)))
                    .unwrap_or_default();

                writedoc!(
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
                                    {image}{emoji}<td align="left" balign="left">{node_label}</td>
                                </tr>
                                {node_desc}
                            </table>>
                            class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
                            {node_width}
                            {node_height}
                            {margin}
                        ]
                    "#
                )?
            }
            GraphStyle::Circle => {
                // `margin` doesn't apply to `plain` shaped nodes, so we use rectangle and use
                // an invisible colour.
                let margin_outer = margins
                    .get(node_id)
                    .copied()
                    .unwrap_or_else(|| graphviz_attrs.margin_node_default().into_inner());
                let margin_inner = match graph_dir {
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
                            margin = "{margin_outer}"
                            class = "{OUTLINE_NONE}"
                            {node_width}
                            {node_height}

                            {node_id} [
                                label = ""
                                class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
                                {margin_inner}
                            ]
                            {node_id}_text [
                                fillcolor="{no_color}"
                                shape="rectangle"
                                {margin_inner}
                                label = <<table
                                    border="0"
                                    cellborder="0"
                                    cellpadding="0"
                                    cellspacing="0"
                                >
                                    <tr>
                                        {image}{emoji}<td align="left" balign="left">{node_label}</td>
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
        let margin = margins
            .get(node_id)
            .copied()
            .unwrap_or_else(|| graphviz_attrs.margin_cluster_default().into_inner());

        writedoc!(
            buffer,
            r#"
                subgraph cluster_{node_id} {{
                    margin = "{margin}"
                    label = <<table
                        border="0"
                        cellborder="0"
                        cellpadding="0"
                        cellspacing="0"
                    >
                        <tr>
                            {image}{emoji}<td align="left" balign="left">{node_label}</td>
                        </tr>
                        {node_desc}
                    </table>>
                    style = "filled,rounded"
                    class = "{OUTLINE_NONE} {node_tailwind_classes}{node_tag_classes}"
                    {node_width}
                    {node_height}
            "#
        )?;

        match graph_dir {
            GraphDir::Horizontal => node_hierarchy
                .iter()
                // Reversing the order we feed nodes to Graphviz dot tends to produce a more natural
                // layout order.
                .rev()
                .try_for_each(|(child_node_id, child_node_hierarchy)| {
                    let node_cluster_args = NodeClusterArgs {
                        info_graph,
                        el_css_classes,
                        tag_el_css_classes_map,
                        theme,
                        node_id: child_node_id,
                        node_hierarchy: child_node_hierarchy,
                        node_widths,
                        node_heights,
                    };

                    node_cluster_internal(node_cluster_args, buffer)
                })?,
            GraphDir::Vertical => {
                node_hierarchy
                    .iter()
                    .try_for_each(|(child_node_id, child_node_hierarchy)| {
                        let node_cluster_args = NodeClusterArgs {
                            info_graph,
                            el_css_classes,
                            tag_el_css_classes_map,
                            theme,
                            node_id: child_node_id,
                            node_hierarchy: child_node_hierarchy,
                            node_widths,
                            node_heights,
                        };

                        node_cluster_internal(node_cluster_args, buffer)
                    })?
            }
        }

        write!(buffer, "}}")?;
    }

    Ok(())
}

fn image(images: &Images, node_image: Option<&ImageId>, node_desc: Option<&str>) -> Option<String> {
    node_image
        .and_then(|image_id| images.get(image_id).map(|image| (image_id, image)))
        .map(|(image_id, image)| {
            let rowspan = if node_desc.is_some() {
                "rowspan=\"2\""
            } else {
                ""
            };

            let GraphvizImage {
                path: _,
                width,
                height,
            } = image;

            // Extra `<td>` is for spacing
            format!(
                "\
                <td \
                    valign=\"top\" \
                    {rowspan} \
                    fixedsize=\"true\" \
                    width=\"{width}\" \
                    height=\"{height}\" \
                >\
                    <img src=\"{image_id}\" />\
                </td>\
                <td \
                    {rowspan} \
                    fixedsize=\"true\" \
                    width=\"10px\" \
                ></td>"
            )
        })
}

fn emoji(
    node_emoji: Option<&str>,
    node_desc: Option<&str>,
    theme: &GraphvizDotTheme,
    node_point_size: u32,
) -> Option<String> {
    node_emoji.map(|emoji| {
        let rowspan = if node_desc.is_some() {
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
                {rowspan}
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
    })
}

struct NodeClusterArgs<'args> {
    info_graph: &'args InfoGraph,
    el_css_classes: &'args ElCssClasses,
    tag_el_css_classes_map: &'args IndexMap<&'args TagId, ElCssClasses>,
    theme: &'args GraphvizDotTheme,
    node_id: &'args NodeId,
    node_hierarchy: &'args NodeHierarchy,
    node_widths: &'args NodeWidths,
    node_heights: &'args NodeHeights,
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

fn edges(
    info_graph: &InfoGraph,
    node_id_to_hierarchy: &std::collections::HashMap<&NodeId, &NodeHierarchy>,
    el_css_classes: &ElCssClasses,
    tag_el_css_classes_map: &IndexMap<&TagId, ElCssClasses>,
) -> String {
    let graphviz_attrs = info_graph.graphviz_attrs();
    let edge_tags_set = info_graph.edge_tags_set();
    let edge_tags_set = &edge_tags_set;

    info_graph
        .edges()
        .iter()
        .map(|(edge_id, [src_node_id, target_node_id])| {
            let edge_desc = info_graph.edge_descs().get(edge_id).map(String::as_str);
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
            let (target_node_id_plain, target_compass_point) = match target_node_id.split_once(':')
            {
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
        .join("\n")
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
    graph_dir: GraphDir,
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
    // `rev()` here makes the tags appear in the correct order for horizontal
    // graphs.
    tags.iter().rev().try_for_each(|(tag_id, tag_name)| {
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
                    style     = invis
                ]
            }}
            "#
        )?;

        Ok(())
    })?;

    if graph_dir == GraphDir::Vertical {
        // Add invisible edge between tags to enforce ordering
        let mut tag_ids_iter = tags.keys();
        if let Some(mut tag_id_current) = tag_ids_iter.next() {
            for tag_id_next in tag_ids_iter {
                writeln!(
                    buffer,
                    "    {tag_id_current} -> {tag_id_next} [style = invis, minlen = 1]"
                )?;
                tag_id_current = tag_id_next;
            }
        }
    }

    writeln!(buffer, "}}")?;

    Ok(())
}
