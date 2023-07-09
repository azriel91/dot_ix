use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for a `NodeId`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `NodeId`s:
///
/// ```rust
/// # use dot_ix::model::{node_id, NodeId};
/// #
/// let _snake = node_id!("snake_case");
/// let _camel = node_id!("camelCase");
/// let _pascal = node_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeId(Cow<'static, str>);

crate::model::common::id_newtype!(NodeId, NodeIdInvalidFmt, node_id);
