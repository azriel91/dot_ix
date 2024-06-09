use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::EdgeId;

/// Each edge's description. `IndexMap<EdgeId, String>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeDescs(IndexMap<EdgeId, String>);

impl EdgeDescs {
    /// Returns a new `EdgeDescs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `EdgeDescs` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, String> {
        self.0
    }
}

impl Deref for EdgeDescs {
    type Target = IndexMap<EdgeId, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeDescs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, String>> for EdgeDescs {
    fn from(inner: IndexMap<EdgeId, String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, String)> for EdgeDescs {
    fn from_iter<I: IntoIterator<Item = (EdgeId, String)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
