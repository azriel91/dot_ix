use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::EdgeId;

/// GraphViz edge minlen. `IndexMap<EdgeId, u32>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`minlen`] attribute for the edge.
///
/// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeMinlens(IndexMap<EdgeId, u32>);

impl EdgeMinlens {
    /// Returns a new `EdgeMinlens` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `EdgeMinlens` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, u32> {
        self.0
    }
}

impl Deref for EdgeMinlens {
    type Target = IndexMap<EdgeId, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeMinlens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, u32>> for EdgeMinlens {
    fn from(inner: IndexMap<EdgeId, u32>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, u32)> for EdgeMinlens {
    fn from_iter<I: IntoIterator<Item = (EdgeId, u32)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
