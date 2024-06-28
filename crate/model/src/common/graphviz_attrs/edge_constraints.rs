use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::EdgeId;

/// GraphViz edge constraint. `IndexMap<EdgeId, bool>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`constraint`]
/// attribute for the edge.
///
/// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeConstraints(IndexMap<EdgeId, bool>);

impl EdgeConstraints {
    /// Returns a new `EdgeConstraints` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `EdgeConstraints` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, bool> {
        self.0
    }
}

impl Deref for EdgeConstraints {
    type Target = IndexMap<EdgeId, bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, bool>> for EdgeConstraints {
    fn from(inner: IndexMap<EdgeId, bool>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, bool)> for EdgeConstraints {
    fn from_iter<I: IntoIterator<Item = (EdgeId, bool)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
