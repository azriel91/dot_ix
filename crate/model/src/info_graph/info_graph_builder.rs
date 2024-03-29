use indexmap::IndexMap;

use crate::{
    common::{Edges, NodeHierarchy, NodeTags, TagId},
    info_graph::{GraphDir, NodeDescs, NodeEmojis, NodeNames, Tag},
    theme::Theme,
};

use crate::info_graph::InfoGraph;

/// Builder for an [`InfoGraph`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InfoGraphBuilder {
    /// Direction of the graph, `vertical` or `horizontal`.
    direction: GraphDir,
    /// Nested nodes.
    hierarchy: NodeHierarchy,
    /// Logical / ordering dependencies.
    edges: Edges,
    /// Each node's name.
    node_names: NodeNames,
    /// Each node's description.
    node_descs: NodeDescs,
    /// Each node's emoji.
    node_emojis: NodeEmojis,
    /// Tags associated with each node.
    node_tags: NodeTags,
    /// Tags to associate with nodes.
    tags: IndexMap<TagId, Tag>,
    /// Theme that controls the CSS classes to add to elements.
    theme: Theme,
    /// Additional CSS to add in the spreadsheet.
    css: String,
}

impl InfoGraphBuilder {
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

    /// Sets the logical / ordering dependencies.
    pub fn with_edges(mut self, edges: Edges) -> Self {
        self.edges = edges;
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

    /// Sets the tags to associate with nodes.
    pub fn with_tags(mut self, tags: IndexMap<TagId, Tag>) -> Self {
        self.tags = tags;
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

    /// Returns an [`InfoGraph`] from the collected parameters.
    pub fn build(self) -> InfoGraph {
        let InfoGraphBuilder {
            direction,
            hierarchy,
            edges,
            node_names,
            node_descs,
            node_emojis,
            node_tags,
            tags,
            theme,
            css,
        } = self;

        InfoGraph {
            direction,
            hierarchy,
            edges,
            node_names,
            node_descs,
            node_emojis,
            node_tags,
            tags,
            theme,
            css,
        }
    }
}
