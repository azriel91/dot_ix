use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::common::AnyId;

/// Unique identifier for an edge, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `EdgeId`s:
///
/// ```rust
/// # use dot_ix::model::{edge_id, EdgeId};
/// #
/// let _snake = edge_id!("snake_case");
/// let _camel = edge_id!("camelCase");
/// let _pascal = edge_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct EdgeId(Cow<'static, str>);

crate::common::id_newtype!(EdgeId, EdgeIdInvalidFmt, edge_id);

impl From<AnyId> for EdgeId {
    fn from(any_id: AnyId) -> Self {
        let id = any_id.into_inner();

        Self(id)
    }
}
