use std::ops::{Deref, DerefMut};

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

/// Warnings detected while computing CSS utility classes. `IndexSet<String>`
/// newtype.
// TODO: Store a list of error enum variants, rather than `String`s.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ThemeWarnings(IndexSet<String>);

impl ThemeWarnings {
    /// Returns a new `ThemeWarnings` list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `ThemeWarnings` list with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexSet::with_capacity(capacity))
    }

    /// Returns the underlying list.
    pub fn into_inner(self) -> IndexSet<String> {
        self.0
    }
}

impl Deref for ThemeWarnings {
    type Target = IndexSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThemeWarnings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexSet<String>> for ThemeWarnings {
    fn from(inner: IndexSet<String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<String> for ThemeWarnings {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(IndexSet::from_iter(iter))
    }
}
