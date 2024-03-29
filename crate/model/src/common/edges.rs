use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{EdgeId, NodeId};

/// Edges between nodes. `IndexMap<EdgeId, [NodeId; 2]>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Edges(IndexMap<EdgeId, [NodeId; 2]>);

impl Edges {
    /// Returns a new `Edges` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `Edges` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, [NodeId; 2]> {
        self.0
    }
}

impl Deref for Edges {
    type Target = IndexMap<EdgeId, [NodeId; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Edges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, [NodeId; 2]>> for Edges {
    fn from(inner: IndexMap<EdgeId, [NodeId; 2]>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, [NodeId; 2])> for Edges {
    fn from_iter<I: IntoIterator<Item = (EdgeId, [NodeId; 2])>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
