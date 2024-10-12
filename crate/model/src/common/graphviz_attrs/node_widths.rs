use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

/// GraphViz node width. `IndexMap<NodeId, f64>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`width`]
/// attribute for the node.
///
/// [`width`]: https://graphviz.org/docs/attrs/width/
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NodeWidths(IndexMap<NodeId, f64>);

impl NodeWidths {
    /// Returns a new `NodeWidths` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeWidths` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, f64> {
        self.0
    }
}

impl Deref for NodeWidths {
    type Target = IndexMap<NodeId, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeWidths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, f64>> for NodeWidths {
    fn from(inner: IndexMap<NodeId, f64>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, f64)> for NodeWidths {
    fn from_iter<I: IntoIterator<Item = (NodeId, f64)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
