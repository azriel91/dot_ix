use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::model::common::{EdgeId, NodeId, TagId};

pub use self::{node::Node, tag::Tag};

mod node;
mod tag;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoGraph {
    /// List of nodes and basic node info.
    nodes: IndexMap<NodeId, Node>,
    /// Logical / ordering dependencies.
    edges: IndexMap<EdgeId, [NodeId; 2]>,
    /// Tags to associate with nodes.
    tags: IndexMap<TagId, Tag>,
    /// Nested nodes.
    children: IndexMap<NodeId, IndexSet<NodeId>>,
    /// Tags associated with each node.
    node_tags: IndexMap<NodeId, IndexSet<TagId>>,
}

impl InfoGraph {
    pub fn nodes(&self) -> &IndexMap<NodeId, Node> {
        &self.nodes
    }

    pub fn edges(&self) -> &IndexMap<EdgeId, [NodeId; 2]> {
        &self.edges
    }

    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    pub fn children(&self) -> &IndexMap<NodeId, IndexSet<NodeId>> {
        &self.children
    }

    pub fn node_tags(&self) -> &IndexMap<NodeId, IndexSet<TagId>> {
        &self.node_tags
    }
}
