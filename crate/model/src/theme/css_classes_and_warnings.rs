use crate::theme::{CssClasses, ThemeWarnings};

/// `CssClasses` for a node/edge, and warnings detected while computing the CSS
/// utility classes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssClassesAndWarnings {
    /// Whether the element is in the normal, focused, hovered, or active state.
    pub css_classes: CssClasses,
    /// Warnings detected while computing CSS utility classes.
    pub theme_warnings: ThemeWarnings,
}

impl<'params> CssClassesAndWarnings {
    /// Returns a new `CssClassesAndWarnings`.
    pub fn new(css_classes: CssClasses, theme_warnings: ThemeWarnings) -> Self {
        Self {
            css_classes,
            theme_warnings,
        }
    }

    /// Returns whether the element is in the normal, focused, hovered, or
    /// active state.
    pub fn css_classes(&self) -> &str {
        &self.css_classes
    }

    /// Returns warnings detected while computing CSS utility classes.
    pub fn theme_warnings(&self) -> &ThemeWarnings {
        &self.theme_warnings
    }
}
