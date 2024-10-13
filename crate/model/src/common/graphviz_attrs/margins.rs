use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{graphviz_attrs::Margin, NodeId};

/// GraphViz node or cluster margins. `IndexMap<NodeId, Margin>` newtype.
///
/// This is only used for GraphViz dot graphs, which sets the [`margin`]
/// attribute for the node / cluster.
///
/// If this is unset, nodes will use the default [`NodeMargin`], and clusters
/// will use the default [`ClusterMargin`].
///
/// [`margin`]: https://graphviz.org/docs/attrs/margin/
/// [`NodeMargin`]: crate::common::graphviz_attrs::NodeMargin
/// [`ClusterMargin`]: crate::common::graphviz_attrs::ClusterMargin
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Margins(IndexMap<NodeId, Margin>);

impl Margins {
    /// Returns a new `Margins` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `Margins` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, Margin> {
        self.0
    }
}

impl Deref for Margins {
    type Target = IndexMap<NodeId, Margin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Margins {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, Margin>> for Margins {
    fn from(inner: IndexMap<NodeId, Margin>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, Margin)> for Margins {
    fn from_iter<I: IntoIterator<Item = (NodeId, Margin)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
