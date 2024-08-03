use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    common::TagId,
    theme::{CssClassMerger, CssClassPartials, CssClasses, TagElCssClasses, ThemeAttr, Themeable},
};

/// Theme to style the generated diagram.
///
/// See the documentation on [`Theme`] for how theme merging is handled, this
/// class applies the same strategy, but styles are specified separately for
/// nodes and edges.
///
/// # Design
///
/// Node styles and edge styles are separated because there are classes which
/// cause undesirable visual conflicts.
///
/// For example, styling the `fill_color` on nodes is desirable, whereas when it
/// is applied to edges, the area that the curve of an edge surrounds is
/// undesirably filled.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct TagTheme {
    /// Whether to merge with the base styles.
    merge_with_base: bool,
    /// CSS utility class partials for nodes.
    node_styles: CssClassPartials,
    /// CSS utility class partials for edges.
    edge_styles: CssClassPartials,
}

impl Default for TagTheme {
    fn default() -> Self {
        Self {
            merge_with_base: true,
            node_styles: Default::default(),
            edge_styles: Default::default(),
        }
    }
}

impl TagTheme {
    /// Returns a new `TagTheme`.
    ///
    /// See [`TagTheme::base`] for the base styles that `dot_ix` ships with.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the base `TagTheme`.
    ///
    /// These are the values used by `dot_ix` for diagrams.
    pub fn base() -> Self {
        let mut tag_theme = Self::default();

        tag_theme.node_styles = {
            let mut node_defaults = CssClassPartials::new();

            node_defaults.insert(ThemeAttr::ShapeColor, "lime".into());
            node_defaults.insert(ThemeAttr::StrokeShade, "500".into());
            node_defaults.insert(ThemeAttr::StrokeWidth, "2".into());
            node_defaults.insert(ThemeAttr::FillShade, "200".into());

            node_defaults
        };

        tag_theme.edge_styles = {
            let mut edge_defaults = CssClassPartials::new();

            edge_defaults.insert(ThemeAttr::ShapeColor, "lime".into());
            edge_defaults.insert(ThemeAttr::StrokeShade, "600".into());
            edge_defaults.insert(ThemeAttr::StrokeWidth, "2".into());
            edge_defaults.insert(ThemeAttr::FillShade, "300".into());

            edge_defaults
        };

        tag_theme
    }

    /// Returns the node styles and edge styles.
    pub fn into_inner(self) -> (CssClassPartials, CssClassPartials) {
        (self.node_styles, self.edge_styles)
    }

    /// Returns whether to merge with the base styles.
    pub fn merge_with_base(&self) -> bool {
        self.merge_with_base
    }

    /// Merges the given overlay tag theme over this tag theme.
    ///
    /// Keys in the overlay tag theme will override the keys from this
    /// tag theme.
    pub fn merge_overlay(mut self, overlay: &TagTheme) -> Self {
        overlay.node_styles.iter().for_each(|(theme_attr, value)| {
            self.node_styles.insert(*theme_attr, value.clone());
        });

        self
    }

    /// Computes the CSS utility classes for nodes and edges.
    pub fn tag_el_css_classes<T>(&self, themeable: &T, tag_id: &TagId) -> TagElCssClasses
    where
        T: Themeable,
    {
        let tag_theme = if self.merge_with_base {
            Cow::Owned(TagTheme::base().merge_overlay(self))
        } else {
            Cow::Borrowed(self)
        };

        TagElCssClasses::new(
            tag_theme.node_el_css_classes(themeable, tag_id),
            tag_theme.edge_el_css_classes(themeable, tag_id),
        )
    }

    fn node_el_css_classes<T>(&self, themeable: &T, tag_id: &TagId) -> CssClasses
    where
        T: Themeable,
    {
        CssClassMerger::node_tag_classes(&self.node_styles, themeable, tag_id)
    }

    fn edge_el_css_classes<T>(&self, themeable: &T, tag_id: &TagId) -> CssClasses
    where
        T: Themeable,
    {
        CssClassMerger::edge_tag_classes(&self.edge_styles, themeable, tag_id)
    }
}
