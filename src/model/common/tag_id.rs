use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for a `TagId`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `TagId`s:
///
/// ```rust
/// # use dot_ix::model::{tag_id, TagId};
/// #
/// let _snake = tag_id!("snake_case");
/// let _camel = tag_id!("camelCase");
/// let _pascal = tag_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagId(Cow<'static, str>);

crate::model::common::id_newtype!(TagId, TagIdInvalidFmt, tag_id);
