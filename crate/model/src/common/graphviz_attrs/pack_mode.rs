use std::{fmt, fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::common::graphviz_attrs::PackModeFlag;

/// The GraphViz [`packMode`] type, used to set the [`packmode`] attribute.
///
/// See also the [`pack.3`] definition and [parsing] code.
///
/// [`packmode`]: https://graphviz.org/docs/attrs/packmode/
/// [`packMode`]: https://graphviz.org/docs/attr-types/packMode/
/// [`pack.3`]: https://gitlab.com/graphviz/graphviz/-/blob/13b87ed889893c07315ef2592b0988038e04027c/lib/pack/pack.3#L13-22
/// [parsing]: https://gitlab.com/graphviz/graphviz/-/blob/13b87ed889893c07315ef2592b0988038e04027c/lib/pack/pack.c#L1173-1214
#[derive(Clone, Debug, PartialEq)]
pub enum PackMode {
    Node,
    Cluster,
    Graph,
    Array {
        flags: Vec<PackModeFlag>,
        number: Option<u32>,
    },
    Aspect(f32),
}

impl Default for PackMode {
    fn default() -> Self {
        Self::Array {
            flags: Vec::new(),
            number: Some(1),
        }
    }
}

impl FromStr for PackMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "node" => Ok(Self::Node),
            "cluster" => Ok(Self::Cluster),
            "graph" => Ok(Self::Graph),
            s if s.starts_with("array") => {
                let array_args = &s["array".len()..];
                let number_start = array_args.find(|c| char::is_ascii_digit(&c));
                let flags_start = if array_args.starts_with('_') {
                    Some(1)
                } else {
                    None
                };
                let flags_end = number_start.unwrap_or_else(|| array_args.len());
                let flags = flags_start
                    .map(|flags_start| {
                        let flag_candidates = &array_args[flags_start..flags_end];
                        flag_candidates.chars().try_fold(
                            Vec::with_capacity(flag_candidates.len()),
                            |mut flags, flag_char| {
                                let pack_mode_flag =
                                    PackModeFlag::from_str(&String::from(flag_char))?;

                                flags.push(pack_mode_flag);

                                Ok::<_, String>(flags)
                            },
                        )
                    })
                    .transpose()?
                    .unwrap_or_default();
                let number = number_start
                    .map(|number_start| {
                        let number_str = &array_args[number_start..];
                        number_str.parse::<u32>().map_err(|_e| {
                            format!(
                                "Failed to parse `{number_str}` as a number for `PackMode::Array`."
                            )
                        })
                    })
                    .transpose()?;

                Ok(Self::Array { flags, number })
            }
            s if s.starts_with("aspect") => {
                let aspect_args = &s["aspect".len()..];
                let aspect_start = aspect_args.find(|c| char::is_ascii_digit(&c));
                let aspect = aspect_start
                    .map(|aspect_start| {
                        let aspect_str = &aspect_args[aspect_start..];
                        aspect_str.parse::<f32>().map_err(|_e| {
                            format!(
                                "Failed to parse `{aspect_str}` as a float for `PackMode::Aspect`."
                            )
                        })
                    })
                    .transpose()?
                    .unwrap_or(1.0);

                Ok(Self::Aspect(aspect))
            }
            _ => Err(format!("Failed to parse `PackMode` from `{s}`.")),
        }
    }
}

impl Display for PackMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackMode::Node => "node".fmt(f),
            PackMode::Cluster => "cluster".fmt(f),
            PackMode::Graph => "graph".fmt(f),
            PackMode::Array { flags, number } => {
                write!(f, "array")?;
                if !flags.is_empty() {
                    write!(f, "_")?;
                    flags.iter().try_for_each(|flag| flag.fmt(f))?;
                }
                if let Some(number) = number {
                    number.fmt(f)?;
                }
                Ok(())
            }
            PackMode::Aspect(aspect) => write!(f, "aspect{aspect}"),
        }
    }
}

impl<'de> Deserialize<'de> for PackMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for PackMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
