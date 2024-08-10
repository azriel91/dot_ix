use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{dot_src_and_styles::GraphvizImage, ImageId};

/// Images available for . `IndexMap<ImageId, GraphvizImage>` newtype.
///
/// # Examples
///
/// Each URL should either be a hyperlink to an image, or a base64 encoded
/// data URL:
///
/// ```yaml
/// hierarchy:
///   dove: {}
///   blue: {}
///
/// images:
///   dove_svg:
///     path: "https://peace.mk/img/dove.svg"
///     width: "50px"
///     height: "50px"
///
///   blue_inline:
///     path: >-
///       data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABkAAAAZBAMAAAA2x5hQAAAAAX
///       NSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAADUExURUeK/z7BOdMAAAAJcEhZcwAADsIAAA
///       7CARUoSoAAAAAOSURBVCjPYxgFNAMMDAABXgABAvs87wAAAABJRU5ErkJggg==
///     width: "50px"
///     height: "50px"
///
/// node_images:
///   dove: dove_svg
///   blue: blue_inline
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Images(IndexMap<ImageId, GraphvizImage>);

impl Images {
    /// Returns a new `Images` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `Images` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<ImageId, GraphvizImage> {
        self.0
    }
}

impl Deref for Images {
    type Target = IndexMap<ImageId, GraphvizImage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Images {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<ImageId, GraphvizImage>> for Images {
    fn from(inner: IndexMap<ImageId, GraphvizImage>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(ImageId, GraphvizImage)> for Images {
    fn from_iter<I: IntoIterator<Item = (ImageId, GraphvizImage)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
