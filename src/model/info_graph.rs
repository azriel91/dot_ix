use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::model::common::{EdgeId, NodeHierarchy, NodeId, TagId, TailwindClasses};

pub use self::{node_info::NodeInfo, tag::Tag};

mod node_info;
mod tag;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoGraph {
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
