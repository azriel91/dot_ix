use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::theme::ThemeAttr;

/// Partial CSS class name for each theme attribute. `IndexMap<ThemeAttr,
/// String>` newtype.
///
/// These are *partial* CSS utility class names as an entry may be
/// `StrokeColorNormal: "amber-600"`, whereas the final CSS class name
/// may be `"[&>path]:stroke-amber-600"`.
///
/// Also, one CSS class partial may used to compute multiple CSS classes, such
/// as `StrokeColor: "amber"` mapping to:
///
/// * `"[&>path]:stroke-amber-600"`
/// * `"[&>path]:focus:stroke-amber-500"`
/// * `"[&>path]:hover:stroke-amber-400"`
/// * `"[&>path]:focus:hover:stroke-amber-400"`
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CssClassPartials(IndexMap<ThemeAttr, String>);

impl CssClassPartials {
    /// Returns a new `CssClassPartials` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `CssClassPartials` map with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<ThemeAttr, String> {
        self.0
    }
}

impl Deref for CssClassPartials {
    type Target = IndexMap<ThemeAttr, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CssClassPartials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<ThemeAttr, String>> for CssClassPartials {
    fn from(inner: IndexMap<ThemeAttr, String>) -> Self {
        Self(inner)
    }
}

impl FromIterator<(ThemeAttr, String)> for CssClassPartials {
    fn from_iter<I: IntoIterator<Item = (ThemeAttr, String)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}
