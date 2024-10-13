use std::fmt;

use serde::{Deserialize, Serialize};

use crate::common::graphviz_attrs::Margin;

/// A node [`margin`], which specifies the space between the nodes in the
/// cluster and the cluster bounding box.
///
/// Defaults to `Margin::Same(8.0)`; Graphviz default: `8.0`.
///
/// May be a single float, or two floats separated by a comma.
///
/// [`margin`]: https://graphviz.org/docs/attrs/margin/
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClusterMargin(pub Margin);

impl ClusterMargin {
    /// Returns a new `ClusterMargin`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `Margin`.
    pub fn into_inner(self) -> Margin {
        self.0
    }
}

impl Default for ClusterMargin {
    fn default() -> Self {
        Self(Margin::Same(8.0))
    }
}

impl From<Margin> for ClusterMargin {
    fn from(margin: Margin) -> Self {
        Self(margin)
    }
}

impl std::ops::DerefMut for ClusterMargin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for ClusterMargin {
    type Target = Margin;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ClusterMargin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
