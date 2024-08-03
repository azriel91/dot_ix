use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{common::TagId, theme::TagTheme};

/// Each tag and the items associated with it. `IndexMap<TagId, TagTheme>`
/// newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagStyles(IndexMap<TagId, TagTheme>);

impl TagStyles {
    /// Returns a new `TagStyles` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `TagStyles` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<TagId, TagTheme> {
        self.0
    }
}

impl Deref for TagStyles {
    type Target = IndexMap<TagId, TagTheme>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TagStyles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<TagId, TagTheme>> for TagStyles {
    fn from(inner: IndexMap<TagId, TagTheme>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(TagId, TagTheme)> for TagStyles {
    fn from_iter<I: IntoIterator<Item = (TagId, TagTheme)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
