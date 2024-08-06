use crate::{
    common::{AnyId, EdgeId, NodeId},
    theme::{ColorParams, CssClassesBuilder, LineParams},
};

/// Types that can be rendered into a CSS compatible format, e.g. SVG or HTML
/// elements.
pub trait Themeable {
    /// Returns the IDs of all nodes of this themeable type.
    fn node_ids(&self) -> impl Iterator<Item = &NodeId>
    where
        Self: Sized;

    /// Appends the CSS classes that sets the outline colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `line_params`: Parameters for the CSS utility class.
    fn node_outline_classes(
        &self,
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    );

    /// Appends the CSS classes that sets the line / border colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `line_params`: Parameters for the CSS utility class.
    fn node_stroke_classes(
        &self,
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    );

    /// Appends the CSS classes that sets the background colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the element
    /// that should actually be styled is a `<path>` element within the
    /// one that has the CSS class, then this should return
    /// `"[&>path]:fill-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `color_params`: Parameters for the CSS utility class.
    fn node_fill_classes(
        &self,
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    );

    /// Returns the IDs of all edges of this themeable type.
    fn edge_ids(&self) -> impl Iterator<Item = &EdgeId>
    where
        Self: Sized;

    /// Appends the CSS classes that sets the outline colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `line_params`: Parameters for the CSS utility class.
    fn edge_outline_classes(
        &self,
        edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    );

    /// Appends the CSS classes that sets the stroke colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the
    /// element that should actually be styled is a `<path>` element within
    /// the one that has the CSS class, then this should return
    /// `"[&>path]:stroke-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `line_params`: Parameters for the CSS utility class.
    fn edge_stroke_classes(
        &self,
        edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    );

    /// Appends the CSS classes that sets the background colour and style.
    ///
    /// For example, if the `color_shade` is `"slate-600"`, and the element
    /// that should actually be styled is a `<path>` element within the
    /// one that has the CSS class, then this should return
    /// `"[&>path]:fill-slate-600"`.
    ///
    /// # Parameters
    ///
    /// * `builder`: The builder to append CSS classes.
    /// * `color_params`: Parameters for the CSS utility class.
    fn edge_fill_classes(
        &self,
        edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    );
}
