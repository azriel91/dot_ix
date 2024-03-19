use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{common::AnyId, theme::CssClasses};

/// CSS classes for each element. `IndexMap<AnyId, CssClasses>` newtype.
///
/// Each value is a space separated string of TailwindCSS compatible utility
/// classes.
///
/// This is computed by the relevant [`Themeable`] implementation for each graph
/// type.
///
/// [`Themeable`]: crate::theme::Themeable
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ElCssClasses(IndexMap<AnyId, CssClasses>);

impl ElCssClasses {
    /// Returns a new `ElCssClasses` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `ElCssClasses` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<AnyId, CssClasses> {
        self.0
    }
}

impl Deref for ElCssClasses {
    type Target = IndexMap<AnyId, CssClasses>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElCssClasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<AnyId, CssClasses>> for ElCssClasses {
    fn from(inner: IndexMap<AnyId, CssClasses>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(AnyId, CssClasses)> for ElCssClasses {
    fn from_iter<I: IntoIterator<Item = (AnyId, CssClasses)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}

impl Extend<(AnyId, CssClasses)> for ElCssClasses {
    fn extend<I: IntoIterator<Item = (AnyId, CssClasses)>>(&mut self, iterable: I) {
        <IndexMap<AnyId, CssClasses> as Extend<(AnyId, CssClasses)>>::extend(&mut self.0, iterable)
    }
}
