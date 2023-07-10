use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::model::common::{EdgeId, NodeHierarchy, NodeId, TagId};

pub use self::{node_info::NodeInfo, tag::Tag};

mod node_info;
mod tag;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoGraph {
    /// Nested nodes.
    hierarchy: NodeHierarchy,
    /// Logical / ordering dependencies.
    edges: IndexMap<EdgeId, [NodeId; 2]>,
    /// List of nodes and basic node info.
    node_infos: IndexMap<NodeId, NodeInfo>,
    /// Tags to associate with nodes.
    tags: IndexMap<TagId, Tag>,
    /// Tags associated with each node.
    node_tags: IndexMap<NodeId, IndexSet<TagId>>,
}

impl InfoGraph {
    pub fn hierarchy(&self) -> &NodeHierarchy {
        &self.hierarchy
    }

    pub fn edges(&self) -> &IndexMap<EdgeId, [NodeId; 2]> {
        &self.edges
    }

    pub fn node_infos(&self) -> &IndexMap<NodeId, NodeInfo> {
        &self.node_infos
    }

    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    pub fn node_tags(&self) -> &IndexMap<NodeId, IndexSet<TagId>> {
        &self.node_tags
    }
}
