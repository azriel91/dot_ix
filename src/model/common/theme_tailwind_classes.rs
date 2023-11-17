use std::ops::{Deref, DerefMut};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::model::common::{TailwindClass, TailwindKey};

/// Map of tailwind keys to tailwind classes to apply for that key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ThemeTailwindClasses(IndexMap<TailwindKey, IndexSet<TailwindClass>>);

impl ThemeTailwindClasses {
    /// Returns a new `TailwindClasses` map without any allocation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `TailwindClasses` map with the given initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying `IndexMap`.
    pub fn into_inner(self) -> IndexMap<TailwindKey, IndexSet<TailwindClass>> {
        self.0
    }
}

impl Deref for ThemeTailwindClasses {
    type Target = IndexMap<TailwindKey, IndexSet<TailwindClass>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThemeTailwindClasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
