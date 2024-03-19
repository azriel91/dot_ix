use crate::theme::CssClasses;

/// Essentially a builder for `CssClasses`.
#[derive(Debug, Default)]
pub struct CssClassesBuilder {
    css_classes: CssClasses,
}

impl CssClassesBuilder {
    /// Returns a new `Formatter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a CSS class to the list of CSS classes.
    pub fn append(&mut self, class: &str) {
        self.css_classes.push_str(class.trim());
        self.css_classes.push(' ');
    }

    pub fn build(self) -> CssClasses {
        let CssClassesBuilder { css_classes } = self;

        CssClasses::from(css_classes.into_inner().trim().to_owned())
    }
}
