use crate::theme::CssClasses;

/// CSS classes for nodes and edges associated with a particular tag.
///
/// There will be one instance of this class for each tag.
///
/// This is computed by the relevant [`Themeable`] implementation for each graph
/// type.
///
/// [`Themeable`]: crate::theme::Themeable
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TagElCssClasses {
    /// The CSS classes to apply to nodes for the tag.
    pub node_css_classes: CssClasses,
    /// The CSS classes to apply to edges for the tag.
    pub edge_css_classes: CssClasses,
}

impl TagElCssClasses {
    /// Returns a new `TagElCssClasses`.
    pub fn new(node_css_classes: CssClasses, edge_css_classes: CssClasses) -> Self {
        Self {
            node_css_classes,
            edge_css_classes,
        }
    }

    /// Returns the underlying `CssClasses` for nodes and edges.
    pub fn into_inner(self) -> (CssClasses, CssClasses) {
        (self.node_css_classes, self.edge_css_classes)
    }
}
