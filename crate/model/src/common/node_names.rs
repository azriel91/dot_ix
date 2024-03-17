use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

/// Each node's name. `IndexMap<NodeId, String>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeNames(IndexMap<NodeId, String>);

impl NodeNames {
    /// Returns a new `NodeNames` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeNames` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, String> {
        self.0
    }
}

impl Deref for NodeNames {
    type Target = IndexMap<NodeId, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, String>> for NodeNames {
    fn from(inner: IndexMap<NodeId, String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, String)> for NodeNames {
    fn from_iter<I: IntoIterator<Item = (NodeId, String)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
