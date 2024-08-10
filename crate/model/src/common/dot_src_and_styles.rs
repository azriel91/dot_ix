use serde::{Deserialize, Serialize};

use crate::theme::ThemeWarnings;

pub use self::{graphviz_image::GraphvizImage, graphviz_opts::GraphvizOpts};

mod graphviz_image;
mod graphviz_opts;

/// Graphviz dot source and CSS styles.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DotSrcAndStyles {
    /// Graphviz dot source to run in `dot`.
    pub dot_src: String,
    /// Tailwind CSS styles to put into `<styles>..</styles>`.
    pub styles: String,
    /// Options to pass to graphviz when rendering.
    pub opts: GraphvizOpts,
    /// Warnings detected while computing CSS utility classes.
    pub theme_warnings: ThemeWarnings,
}

impl DotSrcAndStyles {
    /// Returns a new `DotSrcAndStyles` object.
    pub fn new(
        dot_src: String,
        styles: String,
        opts: GraphvizOpts,
        theme_warnings: ThemeWarnings,
    ) -> Self {
        Self {
            dot_src,
            styles,
            opts,
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

    /// Returns the options to pass to graphviz when rendering.
    pub fn opts(&self) -> &GraphvizOpts {
        &self.opts
    }

    /// Returns the warnings detected while computing CSS utility classes.
    pub fn theme_warnings(&self) -> &ThemeWarnings {
        &self.theme_warnings
    }
}
