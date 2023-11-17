use serde::{Deserialize, Serialize};

use crate::model::common::AnyId;

/// Key for users to specify tailwind styles against individual nodes, edges, or
/// tags.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TailwindKey {
    /// Default styling to apply to all nodes.
    NodeDefaults,
    /// Default styling to apply to all edges.
    EdgeDefaults,
    /// ID of a node, edge, or tag.
    AnyId(AnyId),
}
