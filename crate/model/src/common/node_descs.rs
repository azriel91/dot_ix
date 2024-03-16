use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

/// Each node's description. `IndexMap<NodeId, String>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeDescs(IndexMap<NodeId, String>);

impl NodeDescs {
    /// Returns a new `NodeDescs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeDescs` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, String> {
        self.0
    }
}

impl Deref for NodeDescs {
    type Target = IndexMap<NodeId, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeDescs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, String>> for NodeDescs {
    fn from(inner: IndexMap<NodeId, String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, String)> for NodeDescs {
    fn from_iter<I: IntoIterator<Item = (NodeId, String)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
