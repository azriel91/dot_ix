use std::fmt;

use serde::{Deserialize, Serialize};

/// Whether a node's `width` and `height` are fixed dimensions.
///
/// See [`fixedsize`].
///
/// [`fixedsize`]: https://graphviz.org/docs/attrs/fixedsize/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixedSize {
    /// Nodes are not fixed size, and `width`/`height` indicate their minimum dimensions.
    #[default]
    False,
    /// Nodes are fixed size, and `width`/`height` indicate their maximum dimensions.
    True,
    /// `width` and `height` determine the dimensions of the node's shape, but not its label.
    Shape,
}

impl fmt::Display for FixedSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixedSize::False => write!(f, "false"),
            FixedSize::True => write!(f, "true"),
            FixedSize::Shape => write!(f, "shape"),
        }
    }
}
