use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Direction to lay out the graph nodes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphDir {
    /// The graph is laid out from left to right.
    #[default]
    Horizontal,
    /// The graph is laid out from top to bottom.
    Vertical,
}

impl Display for GraphDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GraphDir::Horizontal => "horizontal".fmt(f),
            GraphDir::Vertical => "vertical".fmt(f),
        }
    }
}
