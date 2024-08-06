use serde::{Deserialize, Serialize};

use crate::theme::ThemeWarnings;

/// Graphviz dot source and CSS styles.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DotSrcAndStyles {
    /// Graphviz dot source to run in `dot`.
    pub dot_src: String,
    /// Tailwind CSS styles to put into `<styles>..</styles>`.
    pub styles: String,
    /// Warnings detected while computing CSS utility classes.
    pub theme_warnings: ThemeWarnings,
}

impl DotSrcAndStyles {
    /// Returns a new `DotSrcAndStyles` object.
    pub fn new(dot_src: String, styles: String, theme_warnings: ThemeWarnings) -> Self {
        Self {
            dot_src,
            styles,
            theme_warnings,
        }
    }

    /// Returns the Graphviz dot source to run in `dot`.
    pub fn dot_src(&self) -> &str {
        &self.dot_src
    }

    /// Returns the tailwind CSS styles to put into `<styles>..</styles>`.
    pub fn styles(&self) -> &str {
        &self.styles
    }

    /// Returns the warnings detected while computing CSS utility classes.
    pub fn theme_warnings(&self) -> &ThemeWarnings {
        &self.theme_warnings
    }
}
