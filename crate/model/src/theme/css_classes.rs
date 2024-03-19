use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// Space-separated CSS utility classes for an element. `String` newtype.
///
/// These are computed for each element in [`Theme::el_css_classes`], and stored
/// in the [`ElCssClasses`] map.
///
/// [`Theme::el_css_classes`]: crate::theme::Theme::el_css_classes
/// [`ElCssClasses`]: crate::theme::ElCssClasses
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CssClasses(String);

impl CssClasses {
    /// Returns a new `CssClasses` string.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `CssClasses` string with the given preallocated
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    /// Returns the underlying string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for CssClasses {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Deref for CssClasses {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CssClasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<str> for CssClasses {
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.0)
    }
}
