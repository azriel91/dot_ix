use std::ops::{Deref, DerefMut};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::common::{NodeId, TagId};

/// Tags associated with each node. `IndexMap<NodeId, IndexSet<TagId>>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeTags(IndexMap<NodeId, IndexSet<TagId>>);

impl NodeTags {
    /// Returns a new `NodeTags` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeTags` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, IndexSet<TagId>> {
        self.0
    }
}

impl Deref for NodeTags {
    type Target = IndexMap<NodeId, IndexSet<TagId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeTags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, IndexSet<TagId>>> for NodeTags {
    fn from(inner: IndexMap<NodeId, IndexSet<TagId>>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, IndexSet<TagId>)> for NodeTags {
    fn from_iter<I: IntoIterator<Item = (NodeId, IndexSet<TagId>)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
