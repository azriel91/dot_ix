use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{ImageId, NodeId};

/// Image to insert for each node. `IndexMap<NodeId, ImageId>` newtype.
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
pub struct NodeImages(IndexMap<NodeId, ImageId>);

impl NodeImages {
    /// Returns a new `NodeImages` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `NodeImages` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<NodeId, ImageId> {
        self.0
    }
}

impl Deref for NodeImages {
    type Target = IndexMap<NodeId, ImageId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeImages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, ImageId>> for NodeImages {
    fn from(inner: IndexMap<NodeId, ImageId>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, ImageId)> for NodeImages {
    fn from_iter<I: IntoIterator<Item = (NodeId, ImageId)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
