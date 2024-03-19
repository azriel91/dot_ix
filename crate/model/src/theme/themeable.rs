use crate::{
    common::{EdgeId, NodeId},
    theme::{CssClasses, CssClassesBuilder, ThemeableParams},
};

/// Types that can be rendered into a CSS compatible format, e.g. SVG or HTML
/// elements.
pub trait Themeable {
    /// Returns the IDs of all nodes of this themeable type.
    fn node_ids(&self) -> &[NodeId];

    /// Returns the CSS classes that sets the line / border colour.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `params`: Parameters for the CSS utility class.
    fn node_stroke_classes<'f>(
        &self,
        builder: &mut CssClassesBuilder,
        params: ThemeableParams<'f>,
    ) -> CssClasses;

    /// Returns the CSS classes that sets the background colour.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the element
    /// that should actually be styled is a `<path>` element within the
    /// one that has the CSS class, then this should return
    /// `"[&>path]:fill-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `params`: Parameters for the CSS utility class.
    fn node_fill_classes<'f>(
        &self,
        builder: &mut CssClassesBuilder,
        params: ThemeableParams<'f>,
    ) -> CssClasses;

    /// Returns the IDs of all edges of this themeable type.
    fn edge_ids(&self) -> &[EdgeId];

    /// Returns the CSS classes that sets the stroke colour.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `params`: Parameters for the CSS utility class.
    fn edge_stroke_classes<'f>(
        &self,
        builder: &mut CssClassesBuilder,
        params: ThemeableParams<'f>,
    ) -> CssClasses;

    /// Returns the CSS classes that sets the background colour.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the element
    /// that should actually be styled is a `<path>` element within the
    /// one that has the CSS class, then this should return
    /// `"[&>path]:fill-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `params`: Parameters for the CSS utility class.
    fn edge_fill_classes<'f>(
        &self,
        builder: &mut CssClassesBuilder,
        params: ThemeableParams<'f>,
    ) -> CssClasses;
}
