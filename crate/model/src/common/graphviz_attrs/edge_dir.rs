use std::fmt;

use serde::{Deserialize, Serialize};

/// The direction of an edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeDir {
    /// An arrow is drawn in the forward direction.
    #[default]
    Forward,
    /// An arrow is drawn in the reverse direction.
    Back,
    /// Arrows are drawn in both directions.
    Both,
    /// Unidirected.
    None,
}

impl fmt::Display for EdgeDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeDir::Forward => write!(f, "forward"),
            EdgeDir::Back => write!(f, "back"),
            EdgeDir::Both => write!(f, "both"),
            EdgeDir::None => write!(f, "none"),
        }
    }
}
