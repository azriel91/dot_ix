use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

/// GraphViz node height. `IndexMap<NodeId, f64>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`height`]
/// attribute for the node.
///
/// [`height`]: https://graphviz.org/docs/attrs/height/
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NodeHeights(IndexMap<NodeId, f64>);

impl NodeHeights {
    /// Returns a new `NodeHeights` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeHeights` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, f64> {
        self.0
    }
}

impl Deref for NodeHeights {
    type Target = IndexMap<NodeId, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeHeights {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, f64>> for NodeHeights {
    fn from(inner: IndexMap<NodeId, f64>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, f64)> for NodeHeights {
    fn from_iter<I: IntoIterator<Item = (NodeId, f64)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
