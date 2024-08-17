use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/// How to render edge lines. See [`splines`].
///
/// [`splines`]: https://graphviz.org/docs/attrs/splines/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Splines {
    /// Edges should be drawn using the layout engine's default.
    ///
    /// For `dot`, this is equivalent to `Spline`.
    #[default]
    Unset,
    /// Edges should not be drawn.
    None,
    /// Edges should be drawn using straight lines.
    Line,
    /// Edges should be drawn with a few straight lines.
    Polyline,
    /// Edges should be drawn with a curved line, with a single curve.
    Curved,
    /// Edges should be drawn with lines that only bend with 90 degree angles.
    Ortho,
    /// Edges should be drawn with a curved line that can curve multiple times.
    Spline,
}

impl FromStr for Splines {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::Unset),
            "none" => Ok(Self::None),
            "line" => Ok(Self::Line),
            "polyline" => Ok(Self::Polyline),
            "curved" => Ok(Self::Curved),
            "ortho" => Ok(Self::Ortho),
            "spline" => Ok(Self::Spline),
            _ => Err(format!(
                "Unable to map `{s}` to a `Splines`. Valid strings are: \
                \"\", \"none\", \"line\", \"polyline\", \"curved\", \"ortho\", \"spline\"."
            )),
        }
    }
}

impl Display for Splines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Splines::Unset => "".fmt(f),
            Splines::None => "none".fmt(f),
            Splines::Line => "line".fmt(f),
            Splines::Polyline => "polyline".fmt(f),
            Splines::Curved => "curved".fmt(f),
            Splines::Ortho => "ortho".fmt(f),
            Splines::Spline => "spline".fmt(f),
        }
    }
}
