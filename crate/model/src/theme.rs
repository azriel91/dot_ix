use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub use self::{
    any_id_or_defaults::AnyIdOrDefaults, css_class_partials::CssClassPartials,
    theme_attr::ThemeAttr,
};

mod any_id_or_defaults;
mod css_class_partials;
mod theme_attr;

/// Theme to style the generated diagram. Map of [`AnyIdOrDefaults`] to
/// [`CssClassPartials`].
///
/// This is a way to simplify what Tailwind CSS classes are provided, as the
/// same styling may need different prefixes depending on the structure of the
/// generated diagram.
///
/// For example, styling the border colour may be `[&>path]:stroke-amber-400`
/// for a dot SVG diagram, but `border-amber-400` for an HTML diagram using
/// `div`s.
///
/// The appropriate Tailwind class will be calculated for each diagram type, and
/// the CSS generated off that.
///
/// Another reason for storing each style in its own key is to allow overriding
/// individual keys without needing to specify all of the classes.
///
/// # Design
///
/// 1. Take theme input.
/// 2. Generate colour based classes for each node / edge.
/// 3. Deep merge all the classes.
///
/// ## Merge Order
///
/// 1. Theme default.
/// 2. Colour override.
/// 3. Node/edge specific override.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme(IndexMap<AnyIdOrDefaults, CssClassPartials>);

impl Theme {
    /// Returns a new `Theme`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `Theme` with the given preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<AnyIdOrDefaults, CssClassPartials> {
        self.0
    }
}

impl Deref for Theme {
    type Target = IndexMap<AnyIdOrDefaults, CssClassPartials>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<AnyIdOrDefaults, CssClassPartials>> for Theme {
    fn from(inner: IndexMap<AnyIdOrDefaults, CssClassPartials>) -> Self {
        Self(inner)
    }
}
