use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::common::AnyId;

/// Unique identifier for an image, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `ImageId`s:
///
/// ```rust
/// # use dot_ix::model::{image_id, ImageId};
/// #
/// let _snake = image_id!("snake_case");
/// let _camel = image_id!("camelCase");
/// let _pascal = image_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImageId(Cow<'static, str>);

id_newtype::id_newtype!(ImageId, ImageIdInvalidFmt, image_id);

impl From<AnyId> for ImageId {
    fn from(any_id: AnyId) -> Self {
        let id = any_id.into_inner();

        Self(id)
    }
}
