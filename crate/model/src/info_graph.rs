use std::collections::{HashMap, VecDeque};

pub use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::{
        EdgeDescs, Edges, GraphvizAttrs, NodeDescs, NodeEmojis, NodeHierarchy, NodeId, NodeNames,
        NodeTags, TagId,
    },
    theme::Theme,
};

pub use self::{graph_dir::GraphDir, graph_style::GraphStyle, tag::Tag};

mod graph_dir;
mod graph_style;
mod tag;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoGraph {
    /// Style of graph to render.
    pub graph_style: GraphStyle,
    /// Direction of the graph, `vertical` or `horizontal`.
    pub direction: GraphDir,
    /// Nested nodes.
    pub hierarchy: NodeHierarchy,
    /// Each node's name.
    pub node_names: NodeNames,
    /// Each node's description.
    pub node_descs: NodeDescs,
    /// Each node's emoji.
    pub node_emojis: NodeEmojis,
    /// Tags associated with each node.
    pub node_tags: NodeTags,
    /// Logical / ordering dependencies.
    pub edges: Edges,
    /// Each edge's description.
    pub edge_descs: EdgeDescs,
    /// Tags to associate with nodes.
    pub tags: IndexMap<TagId, Tag>,
    /// Additional attributes specifically for GraphViz.
    pub graphviz_attrs: GraphvizAttrs,
    /// Theme that controls the CSS classes to add to elements.
    pub theme: Theme,
    /// Additional CSS to add in the spreadsheet.
    pub css: String,
}

impl InfoGraph {
    /// Sets the style of graph to render.
    pub fn with_graph_style(mut self, graph_style: GraphStyle) -> Self {
        self.graph_style = graph_style;
        self
    }

    /// Sets the direction of the graph, `vertical` or `horizontal`.
    pub fn with_direction(mut self, direction: GraphDir) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the nested nodes.
    pub fn with_hierarchy(mut self, hierarchy: NodeHierarchy) -> Self {
        self.hierarchy = hierarchy;
        self
    }

    /// Sets the map of node names.
    pub fn with_node_names(mut self, node_names: NodeNames) -> Self {
        self.node_names = node_names;
        self
    }

    /// Sets the map of node descriptions.
    pub fn with_node_descs(mut self, node_descs: NodeDescs) -> Self {
        self.node_descs = node_descs;
        self
    }

    /// Sets the map of node emojis.
    pub fn with_node_emojis(mut self, node_emojis: NodeEmojis) -> Self {
        self.node_emojis = node_emojis;
        self
    }

    /// Sets the tags associated with each node.
    pub fn with_node_tags(mut self, node_tags: NodeTags) -> Self {
        self.node_tags = node_tags;
        self
    }

    /// Sets the logical / ordering dependencies.
    pub fn with_edges(mut self, edges: Edges) -> Self {
        self.edges = edges;
        self
    }

    /// Sets the map of edge descriptions.
    pub fn with_edge_descs(mut self, edge_descs: EdgeDescs) -> Self {
        self.edge_descs = edge_descs;
        self
    }

    /// Sets the tags to associate with nodes.
    pub fn with_tags(mut self, tags: IndexMap<TagId, Tag>) -> Self {
        self.tags = tags;
        self
    }

    /// Sets the additional attributes specifically for GraphViz.
    pub fn with_graphviz_attrs(mut self, graphviz_attrs: GraphvizAttrs) -> Self {
        self.graphviz_attrs = graphviz_attrs;
        self
    }

    /// Sets the theme that controls the CSS classes to add to elements.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the additional CSS to add in the spreadsheet.
    pub fn with_css(mut self, css: String) -> Self {
        self.css = css;
        self
    }

    /// Returns the style of graph to render.
    pub fn graph_style(&self) -> GraphStyle {
        self.graph_style
    }

    /// Returns the direction of the graph, `vertical` or `horizontal`.
    pub fn direction(&self) -> GraphDir {
        self.direction
    }

    /// Returns the nested nodes.
    pub fn hierarchy(&self) -> &NodeHierarchy {
        &self.hierarchy
    }

    /// Returns a flatten map of the node hierarchy.
    ///
    /// For example, if the hierarchy is:
    ///
    /// ```yaml
    /// a:
    ///   b:
    ///     c: {}
    ///     d: {}
    /// ```
    ///
    /// This returns:
    ///
    /// ```yaml
    /// a:
    ///   b: {}
    /// b:
    ///   c: {}
    ///   d: {}
    /// c: {}
    /// d: {}
    /// ```
    pub fn hierarchy_flat(&self) -> HashMap<&NodeId, &NodeHierarchy> {
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
    }

    /// Returns the map of node names.
    pub fn node_names(&self) -> &NodeNames {
        &self.node_names
    }

    /// Returns the map of node descriptions.
    pub fn node_descs(&self) -> &NodeDescs {
        &self.node_descs
    }

    /// Returns the map of node emojis.
    pub fn node_emojis(&self) -> &NodeEmojis {
        &self.node_emojis
    }

    /// Returns the tags associated with each node.
    pub fn node_tags(&self) -> &NodeTags {
        &self.node_tags
    }

    /// Returns the logical / ordering dependencies.
    pub fn edges(&self) -> &Edges {
        &self.edges
    }

    /// Returns the map of edge descriptions.
    pub fn edge_descs(&self) -> &EdgeDescs {
        &self.edge_descs
    }

    /// Returns the tags to associate with nodes.
    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    /// Returns the additional attributes specifically for GraphViz.
    pub fn graphviz_attrs(&self) -> &GraphvizAttrs {
        &self.graphviz_attrs
    }

    /// Returns the theme that controls the CSS classes to add to elements.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns the additional CSS to add in the spreadsheet.
    pub fn css(&self) -> &str {
        &self.css
    }
}
