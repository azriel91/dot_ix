use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for any entity ID in the graph, `Cow<'static, str>`
/// newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AnyId(Cow<'static, str>);

crate::model::common::id_newtype!(AnyId, AnyIdInvalidFmt, node_id);
