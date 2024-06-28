use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{graphviz_attrs::EdgeDir, EdgeId};

/// GraphViz edge dir. `IndexMap<EdgeId, EdgeDir>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`dir`]
/// attribute for the edge.
///
/// [`dir`]: https://graphviz.org/docs/attrs/dir/
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeDirs(IndexMap<EdgeId, EdgeDir>);

impl EdgeDirs {
    /// Returns a new `EdgeDirs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `EdgeDirs` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<EdgeId, EdgeDir> {
        self.0
    }
}

impl Deref for EdgeDirs {
    type Target = IndexMap<EdgeId, EdgeDir>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeDirs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<EdgeId, EdgeDir>> for EdgeDirs {
    fn from(inner: IndexMap<EdgeId, EdgeDir>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(EdgeId, EdgeDir)> for EdgeDirs {
    fn from_iter<I: IntoIterator<Item = (EdgeId, EdgeDir)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
