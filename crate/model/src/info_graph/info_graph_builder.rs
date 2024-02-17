use indexmap::{IndexMap, IndexSet};

use crate::{
    common::{EdgeId, NodeHierarchy, NodeId, TagId, TailwindClasses},
    info_graph::{GraphDir, NodeInfo, Tag},
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
    edges: IndexMap<EdgeId, [NodeId; 2]>,
    /// List of nodes and basic node info.
    node_infos: IndexMap<NodeId, NodeInfo>,
    /// Tags associated with each node.
    node_tags: IndexMap<NodeId, IndexSet<TagId>>,
    /// Tags to associate with nodes.
    tags: IndexMap<TagId, Tag>,
    /// Tailwind classes to add to nodes with the given tag.
    tailwind_classes: TailwindClasses,
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
    pub fn with_edges(mut self, edges: IndexMap<EdgeId, [NodeId; 2]>) -> Self {
        self.edges = edges;
        self
    }

    /// Sets the list of nodes and basic node info.
    pub fn with_node_infos(mut self, node_infos: IndexMap<NodeId, NodeInfo>) -> Self {
        self.node_infos = node_infos;
        self
    }

    /// Sets the tags associated with each node.
    pub fn with_node_tags(mut self, node_tags: IndexMap<NodeId, IndexSet<TagId>>) -> Self {
        self.node_tags = node_tags;
        self
    }

    /// Sets the tags to associate with nodes.
    pub fn with_tags(mut self, tags: IndexMap<TagId, Tag>) -> Self {
        self.tags = tags;
        self
    }

    /// Sets the tailwind classes to add to nodes with the given tag.
    pub fn with_tailwind_classes(mut self, tailwind_classes: TailwindClasses) -> Self {
        self.tailwind_classes = tailwind_classes;
        self
    }

    /// Returns an [`InfoGraph`] from the collected parameters.
    pub fn build(self) -> InfoGraph {
        let InfoGraphBuilder {
            direction,
            hierarchy,
            edges,
            node_infos,
            node_tags,
            tags,
            tailwind_classes,
        } = self;

        InfoGraph {
            direction,
            hierarchy,
            edges,
            node_infos,
            node_tags,
            tags,
            tailwind_classes,
        }
    }
}
