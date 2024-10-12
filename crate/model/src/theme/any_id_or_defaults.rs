use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use crate::common::AnyId;

/// Key for users to specify tailwind styles against individual nodes, edges, or
/// tags.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AnyIdOrDefaults {
    /// Styles to apply to all nodes.
    NodeDefaults,
    /// Styles to apply to all edges.
    EdgeDefaults,
    /// ID of a node, edge, or tag.
    AnyId(AnyId),
}

impl AnyIdOrDefaults {
    /// Returns the underlying `AnyId` if this holds an ID.
    pub fn any_id(&self) -> Option<&AnyId> {
        if let Self::AnyId(any_id) = self {
            Some(any_id)
        } else {
            None
        }
    }
}

impl From<AnyId> for AnyIdOrDefaults {
    fn from(any_id: AnyId) -> Self {
        Self::AnyId(any_id)
    }
}

impl Serialize for AnyIdOrDefaults {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AnyIdOrDefaults::NodeDefaults => serializer.serialize_str("node_defaults"),
            AnyIdOrDefaults::EdgeDefaults => serializer.serialize_str("edge_defaults"),
            AnyIdOrDefaults::AnyId(any_id) => serializer.serialize_str(any_id),
        }
    }
}

impl<'de> Deserialize<'de> for AnyIdOrDefaults {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyIdOrDefaultsVisitor)
    }
}

struct AnyIdOrDefaultsVisitor;

impl Visitor<'_> for AnyIdOrDefaultsVisitor {
    type Value = AnyIdOrDefaults;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of `node_defaults`, `edge_defaults`, or a node/edge/tag ID")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let any_id_or_defaults = match value {
            "node_defaults" => AnyIdOrDefaults::NodeDefaults,
            "edge_defaults" => AnyIdOrDefaults::EdgeDefaults,
            _ => {
                let any_id = AnyId::try_from(value.to_owned()).map_err(serde::de::Error::custom)?;
                AnyIdOrDefaults::AnyId(any_id)
            }
        };
        Ok(any_id_or_defaults)
    }
}
