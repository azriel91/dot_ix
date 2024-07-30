use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::TagId;

/// Each tag and its name. `IndexMap<TagId, String>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagNames(IndexMap<TagId, String>);

impl TagNames {
    /// Returns a new `TagNames` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `TagNames` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<TagId, String> {
        self.0
    }
}

impl Deref for TagNames {
    type Target = IndexMap<TagId, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TagNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<TagId, String>> for TagNames {
    fn from(inner: IndexMap<TagId, String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(TagId, String)> for TagNames {
    fn from_iter<I: IntoIterator<Item = (TagId, String)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
