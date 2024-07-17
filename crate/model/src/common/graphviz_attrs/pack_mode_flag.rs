use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/// Use when [`PackMode::Array`] is used.
///
/// See the GraphViz [`packmode`] attribute and [`packMode`] type.
///
/// [`PackMode::Array`]: crate::common::graphviz_attrs::PackMode::Array
/// [`packmode`]: https://graphviz.org/docs/attrs/packmode/
/// [`packMode`]: https://graphviz.org/docs/attr-types/packMode/
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PackModeFlag {
    /// Column major order: clusters are laid out left to right.
    C,
    /// For a vertical rankdir (TB, BT), clusters should be top-aligned.
    T,
    /// For a vertical rankdir (TB, BT), clusters should be bottom-aligned.
    B,
    /// For a horizontal rankdir (LR, RL), clusters should be left-aligned.
    L,
    /// For a horizontal rankdir (LR, RL), clusters should be right-aligned.
    R,
    /// User specified order: clusters are laid out based on the cluster's
    /// `sortv` attribute if specified.
    ///
    /// If it isn't specified, `0` is used.
    U,
}

impl FromStr for PackModeFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(Self::C),
            "t" => Ok(Self::T),
            "b" => Ok(Self::B),
            "l" => Ok(Self::L),
            "r" => Ok(Self::R),
            "u" => Ok(Self::U),
            _ => Err(format!(
                "Unable to map `{s}` to a `PackModeFlag`. Valid strings are: \"c\", \"t\", \"b\", \"l\", \"r\", \"u\"."
            )),
        }
    }
}

impl Display for PackModeFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackModeFlag::C => "c".fmt(f),
            PackModeFlag::T => "t".fmt(f),
            PackModeFlag::B => "b".fmt(f),
            PackModeFlag::L => "l".fmt(f),
            PackModeFlag::R => "r".fmt(f),
            PackModeFlag::U => "u".fmt(f),
        }
    }
}
