pub use indexmap::{IndexMap, IndexSet};

use serde::{Deserialize, Serialize};

use crate::common::{Edges, NodeHierarchy, NodeId, TagId, TailwindClasses};

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
    pub(crate) edges: Edges,
    /// List of nodes and basic node info.
    pub(crate) node_infos: IndexMap<NodeId, NodeInfo>,
    /// Tags associated with each node.
    pub(crate) node_tags: IndexMap<NodeId, IndexSet<TagId>>,
    /// Tags to associate with nodes.
    pub(crate) tags: IndexMap<TagId, Tag>,
    /// Tailwind classes to add to nodes with the given tag.
    pub(crate) tailwind_classes: TailwindClasses,
    /// Additional CSS to add in the spreadsheet.
    pub(crate) css: String,
}

impl InfoGraph {
    /// Returns a builder to instantiate an `InfoGraph`.
    pub fn builder() -> InfoGraphBuilder {
        InfoGraphBuilder::default()
    }

    /// Returns the direction of the graph, `vertical` or `horizontal`.
    pub fn direction(&self) -> GraphDir {
        self.direction
    }

    /// Returns the nested nodes.
    pub fn hierarchy(&self) -> &NodeHierarchy {
        &self.hierarchy
    }

    /// Returns the logical / ordering dependencies.
    pub fn edges(&self) -> &Edges {
        &self.edges
    }

    /// Returns the list of nodes and basic node info.
    pub fn node_infos(&self) -> &IndexMap<NodeId, NodeInfo> {
        &self.node_infos
    }

    /// Returns the tags associated with each node.
    pub fn node_tags(&self) -> &IndexMap<NodeId, IndexSet<TagId>> {
        &self.node_tags
    }

    /// Returns the tags to associate with nodes.
    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    /// Returns the tailwind classes to add to nodes with the given tag.
    pub fn tailwind_classes(&self) -> &TailwindClasses {
        &self.tailwind_classes
    }

    /// Returns the additional CSS to add in the spreadsheet.
    pub fn css(&self) -> &str {
        &self.css
    }
}
