use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{GraphvizImage, NodeId};

/// Image to insert for each node. `IndexMap<NodeId, GraphvizImage>` newtype.
///
/// # Examples
///
/// Each path should either be a hyperlink to an image, or a base64 encoded
/// String:
///
/// ```yaml
/// node_id:
///   path: >-
///     data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABkAAAAZBAMAAAA2x5hQAAAAAX
///     NSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAADUExURUeK/z7BOdMAAAAJcEhZcwAADsIAAA
///     7CARUoSoAAAAAOSURBVCjPYxgFNAMMDAABXgABAvs87wAAAABJRU5ErkJggg==
///   width: "50px"
///   height: "50px"
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct NodeImages(IndexMap<NodeId, GraphvizImage>);

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
    pub fn into_inner(self) -> IndexMap<NodeId, GraphvizImage> {
        self.0
    }
}

impl Deref for NodeImages {
    type Target = IndexMap<NodeId, GraphvizImage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeImages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<NodeId, GraphvizImage>> for NodeImages {
    fn from(inner: IndexMap<NodeId, GraphvizImage>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(NodeId, GraphvizImage)> for NodeImages {
    fn from_iter<I: IntoIterator<Item = (NodeId, GraphvizImage)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
