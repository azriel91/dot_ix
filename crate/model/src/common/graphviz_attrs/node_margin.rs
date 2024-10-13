use std::fmt;

use serde::{Deserialize, Serialize};

use crate::common::graphviz_attrs::Margin;

/// A node [`margin`], which specifies space left around the node's label.
///
/// Defaults to `Margin::Different(0.04, 0.04)`; Graphviz default: `0.11,0.055`.
///
/// May be a single float, or two floats separated by a comma.
///
/// [`margin`]: https://graphviz.org/docs/attrs/margin/
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NodeMargin(pub Margin);

impl NodeMargin {
    /// Returns a new `NodeMargin`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `Margin`.
    pub fn into_inner(self) -> Margin {
        self.0
    }
}

impl Default for NodeMargin {
    fn default() -> Self {
        Self(Margin::Different(0.11, 0.055))
    }
}

impl From<Margin> for NodeMargin {
    fn from(margin: Margin) -> Self {
        Self(margin)
    }
}

impl std::ops::DerefMut for NodeMargin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for NodeMargin {
    type Target = Margin;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NodeMargin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
