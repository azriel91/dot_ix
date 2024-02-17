pub use indexmap::{IndexMap, IndexSet};

use serde::{Deserialize, Serialize};

use crate::common::{EdgeId, NodeHierarchy, NodeId, TagId, TailwindClasses};

pub use self::{
    graph_dir::GraphDir, info_graph_builder::InfoGraphBuilder, node_info::NodeInfo, tag::Tag,
};

mod graph_dir;
mod info_graph_builder;
mod node_info;
mod tag;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoGraph {
    /// Direction of the graph, `vertical` or `horizontal`.
    pub(crate) direction: GraphDir,
    /// Nested nodes.
    pub(crate) hierarchy: NodeHierarchy,
    /// Logical / ordering dependencies.
    pub(crate) edges: IndexMap<EdgeId, [NodeId; 2]>,
    /// List of nodes and basic node info.
    pub(crate) node_infos: IndexMap<NodeId, NodeInfo>,
    /// Tags associated with each node.
    pub(crate) node_tags: IndexMap<NodeId, IndexSet<TagId>>,
    /// Tags to associate with nodes.
    pub(crate) tags: IndexMap<TagId, Tag>,
    /// Tailwind classes to add to nodes with the given tag.
    pub(crate) tailwind_classes: TailwindClasses,
}

impl InfoGraph {
    /// Returns a builder to instantiate an `InfoGraph`.
    pub fn builder() -> InfoGraphBuilder {
        InfoGraphBuilder::default()
    }

    pub fn direction(&self) -> GraphDir {
        self.direction
    }

    pub fn hierarchy(&self) -> &NodeHierarchy {
        &self.hierarchy
    }

    pub fn edges(&self) -> &IndexMap<EdgeId, [NodeId; 2]> {
        &self.edges
    }

    pub fn node_infos(&self) -> &IndexMap<NodeId, NodeInfo> {
        &self.node_infos
    }

    pub fn node_tags(&self) -> &IndexMap<NodeId, IndexSet<TagId>> {
        &self.node_tags
    }

    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    pub fn tailwind_classes(&self) -> &TailwindClasses {
        &self.tailwind_classes
    }
}
