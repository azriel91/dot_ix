use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// Warnings detected while computing CSS utility classes. `Vec<String>`
/// newtype.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ThemeWarnings(Vec<String>);

impl ThemeWarnings {
    /// Returns a new `ThemeWarnings` list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `ThemeWarnings` list with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Returns the underlying list.
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}

impl Deref for ThemeWarnings {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThemeWarnings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<String>> for ThemeWarnings {
    fn from(inner: Vec<String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<String> for ThemeWarnings {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}
