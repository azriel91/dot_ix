use std::ops::{Deref, DerefMut};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::common::{EdgeId, TagId};

/// Tags associated with each edge. `IndexMap<EdgeId, IndexSet<TagId>>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeTagsSet(IndexMap<EdgeId, IndexSet<TagId>>);

impl EdgeTagsSet {
    /// Returns a new `EdgeTags` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `EdgeTags` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, IndexSet<TagId>> {
        self.0
    }
}

impl Deref for EdgeTagsSet {
    type Target = IndexMap<EdgeId, IndexSet<TagId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeTagsSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, IndexSet<TagId>>> for EdgeTagsSet {
    fn from(inner: IndexMap<EdgeId, IndexSet<TagId>>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, IndexSet<TagId>)> for EdgeTagsSet {
    fn from_iter<I: IntoIterator<Item = (EdgeId, IndexSet<TagId>)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
