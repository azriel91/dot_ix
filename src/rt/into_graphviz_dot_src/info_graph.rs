use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    fmt::{self, Write},
};

use indexmap::IndexMap;

use crate::{
    model::{
        common::{EdgeId, GraphvizDotTheme, NodeHierarchy, NodeId},
        info_graph::{InfoGraph, NodeInfo},
    },
    rt::IntoGraphvizDotSrc,
};

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
impl IntoGraphvizDotSrc for &InfoGraph {
    fn into(self, theme: &GraphvizDotTheme) -> String {
        let graph_attrs = graph_attrs(theme);
        let node_attrs = node_attrs(theme);
        let edge_attrs = edge_attrs(theme);

        let node_clusters = self
            .hierarchy()
            .iter()
            .map(|(node_id, node_hierarchy)| {
                node_cluster(theme, self.node_infos(), node_id, node_hierarchy)
            })
            .collect::<Vec<String>>()
            .join("\n");

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

        let edges = self
            .edges()
            .iter()
            .map(|(edge_id, [src_node_id, target_node_id])| {
                // We need to find the node_hierarchy for both the the `src_node_id` and
                // `target_node_id`.
                let src_node_hierarchy = node_id_to_hierarchy.get(src_node_id).copied();
                let target_node_hierarchy = node_id_to_hierarchy.get(target_node_id).copied();
                edge(
                    edge_id,
                    src_node_id,
                    src_node_hierarchy,
                    target_node_id,
                    target_node_hierarchy,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "digraph G {{
                {graph_attrs}
                {node_attrs}
                {edge_attrs}

                {node_clusters}

                {edges}
            }}"
        )
    }
}

fn graph_attrs(theme: &GraphvizDotTheme) -> String {
    let plain_text_color = theme.plain_text_color();
    // Note: `margin` is set to 0.1 because some text lies outside the viewport.
    // This may be due to incorrect width calculation for emoji characters, which
    // GraphViz falls back to the space character width.

    let node_point_size = theme.node_point_size();
    format!(
        "\
            compound  = true\n\
            graph [\n\
                margin    = 0.1\n\
                penwidth  = 0\n\
                nodesep   = 0.0\n\
                ranksep   = 0.02\n\
                bgcolor   = \"transparent\"\n\
                fontname  = \"helvetica\"\n\
                fontcolor = \"{plain_text_color}\"\n\
                fontsize  = {node_point_size}\n\
                rankdir   = TB\n\
            ]\n\
        "
    )
}

fn node_attrs(theme: &GraphvizDotTheme) -> String {
    let node_text_color = theme.node_text_color();
    let node_point_size = theme.node_point_size();
    format!(
        "\
            node [\n\
                fontcolor = \"{node_text_color}\"\n\
                fontname  = \"liberationmono\"\n\
                fontsize  = {node_point_size}\n\
                shape     = \"rect\"\n\
                style     = \"rounded,filled\"\n\
                width     = 0.3\n\
                height    = 0.3\n\
                margin    = 0.04\n\
                color     = \"#9999aa\"\n\
                fillcolor = \"#ddddf5\"\n\
            ]\n\
        "
    )
}

fn edge_attrs(theme: &GraphvizDotTheme) -> String {
    let edge_color = theme.edge_color();
    let plain_text_color = theme.plain_text_color();
    format!(
        "\
            edge [\n\
                arrowsize = 0.7\n\
                color     = \"{edge_color}\"\n\
                fontcolor = \"{plain_text_color}\"\n\
            ]\n\
        "
    )
}

fn node_cluster(
    theme: &GraphvizDotTheme,
    node_infos: &IndexMap<NodeId, NodeInfo>,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
) -> String {
    let mut buffer = String::with_capacity(1024);

    node_cluster_internal(theme, node_infos, node_id, node_hierarchy, &mut buffer)
        .expect("Failed to write node_cluster string.");

    buffer
}

fn node_cluster_internal(
    theme: &GraphvizDotTheme,
    node_infos: &IndexMap<NodeId, NodeInfo>,
    node_id: &NodeId,
    node_hierarchy: &NodeHierarchy,
    buffer: &mut String,
) -> fmt::Result {
    let node_point_size = theme.node_point_size();
    let node_info = node_infos.get(node_id);
    let node_label = node_info.map(NodeInfo::name).unwrap_or(&node_id);
    let node_desc = node_info
        .and_then(NodeInfo::desc)
        .map(|desc| desc.replace("\n", "<br />"))
        .map(|desc| format!("<tr><td balign=\"left\">{desc}</td></tr>"));

    let emoji = node_info.and_then(NodeInfo::emoji).map(|emoji| {
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
        format!(
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
    let classes = "\
            [&>path]:fill-slate-300 \
            [&>path]:stroke-1 \
            [&>path]:stroke-slate-600 \
            [&>path]:hover:fill-slate-200 \
            [&>path]:hover:stroke-slate-600 \
            [&>path]:hover:stroke-2 \
            cursor-pointer \
        ";

    if node_hierarchy.is_empty() {
        write!(
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
                    class = "{classes}"
                ]
            "#
        )?;
    } else {
        write!(
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
                    class = "{classes}"
            "#
        )?;

        node_hierarchy
            .iter()
            .try_for_each(|(child_node_id, child_node_hierarchy)| {
                node_cluster_internal(
                    theme,
                    node_infos,
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
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy
            .get(child_node_id)
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(|node_hierarchy| node_hierarchy.last())
        {
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
        while let Some((next_node_id, next_node_hierarchy)) = child_node_hierarchy
            .get(child_node_id)
            .filter(|node_hierarchy| !node_hierarchy.is_empty())
            .and_then(|node_hierarchy| node_hierarchy.first())
        {
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

    format!(
        r#"{edge_src_node_id} -> {edge_target_node_id} [id = "{edge_id}", minlen = 9 {ltail} {lhead}]"#
    )
}
