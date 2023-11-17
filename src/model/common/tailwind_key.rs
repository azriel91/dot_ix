use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use crate::model::common::AnyId;

/// Key for users to specify tailwind styles against individual nodes, edges, or
/// tags.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TailwindKey {
    /// Default styling to apply to all nodes.
    NodeDefaults,
    /// Default styling to apply to all edges.
    EdgeDefaults,
    /// ID of a node, edge, or tag.
    AnyId(AnyId),
}

impl Serialize for TailwindKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TailwindKey::NodeDefaults => serializer.serialize_str("node_defaults"),
            TailwindKey::EdgeDefaults => serializer.serialize_str("edge_defaults"),
            TailwindKey::AnyId(any_id) => serializer.serialize_str(any_id),
        }
    }
}

impl<'de> Deserialize<'de> for TailwindKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TailwindKeyVisitor)
    }
}

struct TailwindKeyVisitor;

impl<'de> Visitor<'de> for TailwindKeyVisitor {
    type Value = TailwindKey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of `node_defaults`, `edge_defaults`, or a node/edge/tag ID")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let tailwind_key = match value {
            "node_defaults" => TailwindKey::NodeDefaults,
            "edge_defaults" => TailwindKey::EdgeDefaults,
            _ => {
                let any_id = AnyId::try_from(value.to_owned()).map_err(serde::de::Error::custom)?;
                TailwindKey::AnyId(any_id)
            }
        };
        Ok(tailwind_key)
    }
}
