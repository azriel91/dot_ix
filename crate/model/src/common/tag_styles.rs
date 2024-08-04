use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{common::TagId, theme::ThemeStyles};

/// Each tag and styles for the items associated with it. `IndexMap<TagId,
/// ThemeStyles>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagStyles(IndexMap<TagId, ThemeStyles>);

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
    pub fn into_inner(self) -> IndexMap<TagId, ThemeStyles> {
        self.0
    }
}

impl Deref for TagStyles {
    type Target = IndexMap<TagId, ThemeStyles>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TagStyles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<TagId, ThemeStyles>> for TagStyles {
    fn from(inner: IndexMap<TagId, ThemeStyles>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(TagId, ThemeStyles)> for TagStyles {
    fn from_iter<I: IntoIterator<Item = (TagId, ThemeStyles)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
