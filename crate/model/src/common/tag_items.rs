use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{AnyId, TagId};

/// Each tag and the items associated with it. `IndexMap<TagId, Vec<AnyId>>`
/// newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagItems(IndexMap<TagId, Vec<AnyId>>);

impl TagItems {
    /// Returns a new `TagItems` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `TagItems` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<TagId, Vec<AnyId>> {
        self.0
    }
}

impl Deref for TagItems {
    type Target = IndexMap<TagId, Vec<AnyId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TagItems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<TagId, Vec<AnyId>>> for TagItems {
    fn from(inner: IndexMap<TagId, Vec<AnyId>>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(TagId, Vec<AnyId>)> for TagItems {
    fn from_iter<I: IntoIterator<Item = (TagId, Vec<AnyId>)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
