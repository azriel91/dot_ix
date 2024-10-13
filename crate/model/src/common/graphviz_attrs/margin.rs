use std::{fmt, fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Node or cluster [`margin`]s.
///
/// May be a single float, or two floats separated by a comma.
///
/// [`margin`]: https://graphviz.org/docs/attrs/margin/
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Margin {
    /// Margins on both left/right and top/bottom are the same.
    Same(f64),
    /// Margins for left/right are different to top/bottom.
    Different(f64, f64),
}

impl FromStr for Margin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((margin_x, margin_y)) => {
                let margin_x = margin_x.parse::<f64>().map_err(|_e| {
                    format!(
                        "Failed to parse `{margin_x}` as a margin. \
                        Please use a floating point number such as 0.5."
                    )
                })?;
                let margin_y = margin_y.parse::<f64>().map_err(|_e| {
                    format!(
                        "Failed to parse `{margin_y}` as a margin. \
                        Please use a floating point number such as 0.5."
                    )
                })?;

                Ok(Self::Different(margin_x, margin_y))
            }
            None => {
                let margin = s.parse::<f64>().map_err(|_e| {
                    format!(
                        "Failed to parse `{s}` as a margin. \
                        Please use a floating point number such as 0.5."
                    )
                })?;
                Ok(Self::Same(margin))
            }
        }
    }
}

impl Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Margin::Same(margin) => margin.fmt(f),
            Margin::Different(margin_x, margin_y) => write!(f, "{margin_x},{margin_y}"),
        }
    }
}

impl<'de> Deserialize<'de> for Margin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for Margin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
