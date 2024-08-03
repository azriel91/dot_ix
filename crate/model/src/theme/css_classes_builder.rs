use crate::theme::{CssClasses, StyleFor};

/// Essentially a builder for `CssClasses`.
#[derive(Debug, Default)]
pub struct CssClassesBuilder<'tag_id> {
    /// The purpose of CSS classes' styles -- element regular styles, or when a
    /// tag is interacted with.
    style_for: StyleFor<'tag_id>,
    /// The accumulated CSS classes.
    css_classes: CssClasses,
}

impl<'tag_id> CssClassesBuilder<'tag_id> {
    /// Returns a new `Formatter`.
    pub fn new(style_for: StyleFor<'tag_id>) -> Self {
        Self {
            style_for,
            css_classes: CssClasses::default(),
        }
    }

    /// Appends a CSS class to the list of CSS classes.
    pub fn append(&mut self, class: &str) -> &mut Self {
        match self.style_for {
            StyleFor::Regular => {}
            StyleFor::TagFocus(tag_id) => {
                self.css_classes.push_str("peer-focus/");
                self.css_classes.push_str(tag_id.as_str());
                self.css_classes.push(':');
            }
        }

        self.css_classes.push_str(class.trim());
        self.css_classes.push(' ');

        self
    }

    pub fn build(self) -> CssClasses {
        let CssClassesBuilder {
            style_for: _,
            css_classes,
        } = self;

        CssClasses::from(css_classes.into_inner().trim().to_owned())
    }
}
