use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::theme::{AnyIdOrDefaults, CssClassPartials};

/// CSS utility class partials for each element. `IndexMap<AnyIdOrDefaults,
/// CssClassPartials>` newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ThemeStyles(IndexMap<AnyIdOrDefaults, CssClassPartials>);

impl ThemeStyles {
    /// Returns a new `ThemeStyles` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `ThemeStyles` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<AnyIdOrDefaults, CssClassPartials> {
        self.0
    }
}

impl Deref for ThemeStyles {
    type Target = IndexMap<AnyIdOrDefaults, CssClassPartials>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThemeStyles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<AnyIdOrDefaults, CssClassPartials>> for ThemeStyles {
    fn from(inner: IndexMap<AnyIdOrDefaults, CssClassPartials>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(AnyIdOrDefaults, CssClassPartials)> for ThemeStyles {
    fn from_iter<I: IntoIterator<Item = (AnyIdOrDefaults, CssClassPartials)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
