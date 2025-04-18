use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::common::{EdgeId, NodeId, TagId};

/// Unique identifier for any entity ID in the graph, `Cow<'static, str>`
/// newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AnyId(Cow<'static, str>);

id_newtype::id_newtype!(AnyId, AnyIdInvalidFmt, node_id);

impl From<NodeId> for AnyId {
    fn from(node_id: NodeId) -> Self {
        let id = node_id.into_inner();

        Self(id)
    }
}

impl From<EdgeId> for AnyId {
    fn from(edge_id: EdgeId) -> Self {
        let id = edge_id.into_inner();

        Self(id)
    }
}

impl From<TagId> for AnyId {
    fn from(tag_id: TagId) -> Self {
        let id = tag_id.into_inner();

        Self(id)
    }
}
