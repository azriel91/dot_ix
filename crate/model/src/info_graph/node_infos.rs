use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{common::NodeId, info_graph::NodeInfo};

/// Each node's emoji, name, and description. `IndexMap<NodeId,
/// NodeInfo>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeInfos(IndexMap<NodeId, NodeInfo>);

impl NodeInfos {
    /// Returns a new `NodeInfos` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeInfos` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, NodeInfo> {
        self.0
    }
}

impl Deref for NodeInfos {
    type Target = IndexMap<NodeId, NodeInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeInfos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, NodeInfo>> for NodeInfos {
    fn from(inner: IndexMap<NodeId, NodeInfo>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, NodeInfo)> for NodeInfos {
    fn from_iter<I: IntoIterator<Item = (NodeId, NodeInfo)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
