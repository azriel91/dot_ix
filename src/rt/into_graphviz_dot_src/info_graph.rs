use crate::{
    model::{
        common::{EdgeId, GraphvizDotTheme, NodeId},
        InfoGraph, Node,
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
impl IntoGraphvizDotSrc for &InfoGraph {
    fn into(self, theme: &GraphvizDotTheme) -> String {
        let graph_attrs = graph_attrs(theme);
        let node_attrs = node_attrs(theme);
        let edge_attrs = edge_attrs(theme);

        // TODO: group nodes into node cluster hierarchy, and generate clusters based on
        // those hierarchies.
        //
        // Actually, even better, the node hierarchy should be the top level grouping,
        // and the node info in its own attribute map.

        let node_clusters = self
            .nodes()
            .iter()
            .map(|(node_id, node)| node_cluster(theme, node_id, node))
            .collect::<Vec<String>>()
            .join("\n");

        let edges = self
            .edges()
            .iter()
            .map(|(edge_id, [src_node_id, target_node_id])| {
                edge(edge_id, src_node_id, target_node_id)
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
    format!(
        "\
            graph [\n\
                margin    = 0.1\n\
                penwidth  = 0\n\
                nodesep   = 0.0\n\
                ranksep   = 0.02\n\
                bgcolor   = \"transparent\"\n\
                fontname  = \"helvetica\"\n\
                fontcolor = \"{plain_text_color}\"\n\
                splines   = line\n\
                rankdir   = LR\n\
            ]\n\
        "
    )
}

fn node_attrs(theme: &GraphvizDotTheme) -> String {
    let node_text_color = theme.node_text_color();
    format!(
        "\
            node [\n\
                fontcolor = \"{node_text_color}\"\n\
                fontname  = \"monospace\"\n\
                fontsize  = 12\n\
                shape     = \"circle\"\n\
                style     = \"filled\"\n\
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

fn node_cluster(theme: &GraphvizDotTheme, node_id: &NodeId, node: &Node) -> String {
    let node_label = node.name();
    let plain_text_color = theme.plain_text_color();
    let classes = "\
            [&>ellipse]:fill-slate-300 \
            [&>ellipse]:stroke-1 \
            [&>ellipse]:stroke-slate-600 \
            [&>ellipse]:hover:fill-slate-200 \
            [&>ellipse]:hover:stroke-slate-600 \
            [&>ellipse]:hover:stroke-2 \
            cursor-pointer \
        ";
    format!(
        r#"
            subgraph cluster_{node_id} {{
                {node_id} [label = "" class = "{classes}"]
                {node_id}_text [
                    shape="plain"
                    style=""
                    fontcolor="{plain_text_color}"
                    label = <<table
                        border="0"
                        cellborder="0"
                        cellpadding="0">
                        <tr>
                            <td><font point-size="15">📥</font></td>
                            <td balign="left">{node_label}</td>
                        </tr>
                    </table>>
                ]
            }}
        "#
    )
}

fn edge(edge_id: &EdgeId, src_node_id: &NodeId, target_node_id: &NodeId) -> String {
    format!(r#"{src_node_id} -> {target_node_id} [id = "{edge_id}", minlen = 9]"#)
}
