use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::common::{AnyId, EdgeId, NodeId, TagId};

pub use self::{
    any_id_or_defaults::AnyIdOrDefaults, color_params::ColorParams,
    css_class_merger::CssClassMerger, css_class_partials::CssClassPartials,
    css_classes::CssClasses, css_classes_and_warnings::CssClassesAndWarnings,
    css_classes_builder::CssClassesBuilder, el_css_classes::ElCssClasses,
    highlight_state::HighlightState, stroke_params::StrokeParams, style_for::StyleFor,
    theme_attr::ThemeAttr, theme_styles::ThemeStyles, theme_warnings::ThemeWarnings,
    themeable::Themeable,
};

mod any_id_or_defaults;
mod color_params;
mod css_class_merger;
mod css_class_partials;
mod css_classes;
mod css_classes_and_warnings;
mod css_classes_builder;
mod el_css_classes;
mod highlight_state;
mod stroke_params;
mod style_for;
mod theme_attr;
mod theme_styles;
mod theme_warnings;
mod themeable;

/// Theme to style the generated diagram.
///
/// This is a way to simplify what Tailwind CSS classes are provided, as the
/// same styling may need different prefixes depending on the structure of the
/// generated diagram.
///
/// For example, styling the border colour may be `[&>path]:stroke-amber-400`
/// for a dot SVG diagram, but `border-amber-400` for an HTML diagram using
/// `div`s.
///
/// The appropriate Tailwind class will be calculated for each diagram type, and
/// the CSS generated off that.
///
/// Another reason for storing each style in its own key is to allow overriding
/// individual keys without needing to specify all of the classes.
///
/// # Design
///
/// 1. Take theme input.
/// 2. Generate colour based classes for each node / edge.
/// 3. Deep merge all the classes.
///
/// ## Merge Order
///
/// 1. Theme default.
/// 2. Colour override.
/// 3. Node/edge specific override.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Theme {
    /// Whether to merge with the base styles.
    pub merge_with_base: bool,
    /// CSS utility class partials for each element.
    pub styles: ThemeStyles,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            merge_with_base: true,
            styles: Default::default(),
        }
    }
}

impl Theme {
    /// Returns a new `Theme`.
    ///
    /// See [`Theme::base`] for the base styles that `dot_ix` ships with.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the base `Theme`.
    ///
    /// These are the values used by `dot_ix` for diagrams.
    pub fn base() -> Self {
        let mut theme = Self::default();

        theme.insert(AnyIdOrDefaults::NodeDefaults, {
            let mut node_defaults = CssClassPartials::new();
            node_defaults.insert(ThemeAttr::Cursor, "pointer".into());

            node_defaults.insert(ThemeAttr::Padding, "1.5".into());
            node_defaults.insert(ThemeAttr::ShapeColor, "slate".into());

            node_defaults.insert(ThemeAttr::FillShadeNormal, "300".into());
            node_defaults.insert(ThemeAttr::FillShadeFocus, "200".into());
            node_defaults.insert(ThemeAttr::FillShadeHover, "100".into());
            node_defaults.insert(ThemeAttr::FillShadeActive, "200".into());

            node_defaults.insert(ThemeAttr::StrokeShadeNormal, "600".into());
            node_defaults.insert(ThemeAttr::StrokeShadeFocus, "500".into());
            node_defaults.insert(ThemeAttr::StrokeShadeHover, "400".into());
            node_defaults.insert(ThemeAttr::StrokeShadeActive, "500".into());

            node_defaults.insert(ThemeAttr::StrokeWidth, "1".into());
            node_defaults.insert(ThemeAttr::StrokeStyle, "solid".into());

            node_defaults.insert(ThemeAttr::OutlineColor, "blue".into());
            node_defaults.insert(ThemeAttr::OutlineShadeFocus, "500".into());
            node_defaults.insert(ThemeAttr::OutlineWidth, "2".into());
            node_defaults.insert(ThemeAttr::OutlineStyleFocus, "dashed".into());

            node_defaults
        });

        theme.insert(AnyIdOrDefaults::EdgeDefaults, {
            let mut edge_defaults = CssClassPartials::new();
            edge_defaults.insert(ThemeAttr::Cursor, "pointer".into());

            edge_defaults.insert(ThemeAttr::ShapeColor, "slate".into());

            edge_defaults.insert(ThemeAttr::FillShadeNormal, "800".into());
            edge_defaults.insert(ThemeAttr::FillShadeFocus, "700".into());
            edge_defaults.insert(ThemeAttr::FillShadeHover, "600".into());
            edge_defaults.insert(ThemeAttr::FillShadeActive, "700".into());

            edge_defaults.insert(ThemeAttr::StrokeShadeNormal, "900".into());
            edge_defaults.insert(ThemeAttr::StrokeShadeFocus, "800".into());
            edge_defaults.insert(ThemeAttr::StrokeShadeHover, "700".into());
            edge_defaults.insert(ThemeAttr::StrokeShadeActive, "800".into());

            edge_defaults.insert(ThemeAttr::StrokeWidth, "1".into());
            edge_defaults.insert(ThemeAttr::StrokeStyle, "solid".into());

            edge_defaults.insert(ThemeAttr::OutlineColor, "blue".into());
            edge_defaults.insert(ThemeAttr::OutlineShadeFocus, "500".into());
            edge_defaults.insert(ThemeAttr::OutlineWidth, "2".into());
            edge_defaults.insert(ThemeAttr::OutlineStyleFocus, "dashed".into());

            edge_defaults
        });

        theme
    }

    /// Returns the base `Theme` for a tag.
    ///
    /// These will be merged over the user's specified theme.
    pub fn tag_base() -> Self {
        let mut theme = Self::default();

        theme.insert(AnyIdOrDefaults::NodeDefaults, {
            let mut node_defaults = CssClassPartials::new();

            node_defaults.insert(ThemeAttr::ShapeColor, "lime".into());
            node_defaults.insert(ThemeAttr::StrokeShade, "500".into());
            node_defaults.insert(ThemeAttr::StrokeWidth, "2".into());
            // TODO: pass in the normal `Theme` and use the stroke style from that?
            node_defaults.insert(ThemeAttr::StrokeStyle, "solid".into());
            node_defaults.insert(ThemeAttr::FillShade, "200".into());

            node_defaults
        });
        let mut theme = Self::default();

        theme.insert(AnyIdOrDefaults::EdgeDefaults, {
            let mut edge_defaults = CssClassPartials::new();

            edge_defaults.insert(ThemeAttr::ShapeColor, "lime".into());
            edge_defaults.insert(ThemeAttr::StrokeShade, "600".into());
            edge_defaults.insert(ThemeAttr::StrokeWidth, "2".into());
            // TODO: pass in the normal `Theme` and use the stroke style from that?
            edge_defaults.insert(ThemeAttr::StrokeStyle, "solid".into());
            edge_defaults.insert(ThemeAttr::FillShade, "300".into());

            edge_defaults
        });

        theme
    }

    /// Returns a new `Theme` with the given preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            merge_with_base: true,
            styles: ThemeStyles::with_capacity(capacity),
        }
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> ThemeStyles {
        self.styles
    }

    /// Returns whether to merge with the base styles.
    pub fn merge_with_base(&self) -> bool {
        self.merge_with_base
    }

    /// Merges the given overlay theme over this theme.
    ///
    /// Keys in the overlay theme will override the keys from this theme.
    pub fn merge_overlay(mut self, overlay: &Theme) -> Self {
        overlay
            .styles
            .iter()
            .for_each(|(any_id_or_defaults, css_class_partials)| {
                if let Some(existing_partials) = self.styles.get_mut(any_id_or_defaults) {
                    existing_partials.extend(
                        css_class_partials
                            .iter()
                            .map(|(theme_attr, value)| (*theme_attr, value.clone())),
                    );
                } else {
                    self.styles
                        .insert(any_id_or_defaults.clone(), css_class_partials.clone());
                }
            });

        self
    }

    /// Computes the CSS utility classes for each element.
    ///
    /// The [`CssClasses`] produced will contain an entry for each node / edge
    /// ID from the themeable type.
    pub fn el_css_classes<T>(&self, themeable: &T) -> (ElCssClasses, ThemeWarnings)
    where
        T: Themeable,
    {
        let theme = if self.merge_with_base {
            Cow::Owned(Theme::base().merge_overlay(self))
        } else {
            Cow::Borrowed(self)
        };

        theme
            .node_el_css_classes(themeable)
            .chain(theme.edge_el_css_classes(themeable))
            .fold(
                (
                    ElCssClasses::with_capacity(
                        themeable.node_ids().count() + themeable.edge_ids().count(),
                    ),
                    ThemeWarnings::new(),
                ),
                |(mut el_css_classes, mut theme_warnings_acc),
                 (any_id, css_classes_and_warnings)| {
                    let CssClassesAndWarnings {
                        css_classes,
                        theme_warnings,
                    } = css_classes_and_warnings;

                    el_css_classes.insert(any_id, css_classes);
                    theme_warnings_acc.extend(theme_warnings.into_inner());

                    (el_css_classes, theme_warnings_acc)
                },
            )
    }

    fn node_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
    ) -> impl Iterator<Item = (AnyId, CssClassesAndWarnings)> + 'f
    where
        T: Themeable,
    {
        let node_class_partials_defaults = self.get(&AnyIdOrDefaults::NodeDefaults);
        themeable.node_ids().filter_map(move |node_id| {
            let node_class_partials_specified = self.node_class_partials_specified(node_id);

            let any_id = Some(AnyId::from(node_id.clone()));
            let node_classes_and_warnings = CssClassMerger::node_classes(
                node_class_partials_defaults,
                node_class_partials_specified,
                themeable,
            );

            any_id.map(|any_id| (any_id, node_classes_and_warnings))
        })
    }

    fn node_class_partials_specified(&self, node_id: &NodeId) -> Option<&CssClassPartials> {
        self.iter()
            .find_map(|(any_id_or_defaults, css_class_partials)| {
                any_id_or_defaults
                    .any_id()
                    .filter(|any_id| any_id.as_str() == node_id.as_str())
                    .map(|_| css_class_partials)
            })
    }

    fn edge_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
    ) -> impl Iterator<Item = (AnyId, CssClassesAndWarnings)> + 'f
    where
        T: Themeable,
    {
        let edge_class_partials_defaults = self.get(&AnyIdOrDefaults::EdgeDefaults);

        themeable.edge_ids().filter_map(move |edge_id| {
            let edge_class_partials_specified = self.edge_class_partials_specified(edge_id);

            let any_id = Some(AnyId::from(edge_id.clone()));
            let edge_classes_and_warnings = CssClassMerger::edge_classes(
                edge_class_partials_defaults,
                edge_class_partials_specified,
                themeable,
            );

            any_id.map(|any_id| (any_id, edge_classes_and_warnings))
        })
    }

    fn edge_class_partials_specified(&self, edge_id: &EdgeId) -> Option<&CssClassPartials> {
        self.iter()
            .find_map(|(any_id_or_defaults, css_class_partials)| {
                any_id_or_defaults
                    .any_id()
                    .filter(|any_id| any_id.as_str() == edge_id.as_str())
                    .map(|_| css_class_partials)
            })
    }

    /// Computes the CSS utility classes for nodes and edges.
    pub fn tag_el_css_classes<T>(
        &self,
        themeable: &T,
        diagram_theme: &Theme,
        tag_id: &TagId,
    ) -> (ElCssClasses, ThemeWarnings)
    where
        T: Themeable,
    {
        // Note: The order of whether `diagram_theme` should be merged over the
        // `tag_base` can be debated:
        //
        // * If we merge the `tag_base` over `diagram_theme`, we may override desired
        //   stroke styling.
        // * If we merge the `diagram_theme` over `tag_base`, we miss the fill colouring
        //   of the theme.
        let tag_theme = if self.merge_with_base {
            Cow::<Theme>::Owned(
                // diagram_theme
                //     .clone()
                //     .merge_overlay(&Theme::tag_base().merge_overlay(self)),
                Theme::tag_base().merge_overlay(self),
            )
        } else {
            Cow::Owned(diagram_theme.clone().merge_overlay(self))
        };

        tag_theme
            .node_tag_el_css_classes(themeable, tag_id)
            .chain(tag_theme.edge_tag_el_css_classes(themeable, tag_id))
            .fold(
                (
                    ElCssClasses::with_capacity(
                        themeable.node_ids().count() + themeable.edge_ids().count(),
                    ),
                    ThemeWarnings::new(),
                ),
                |(mut el_css_classes, mut theme_warnings_acc),
                 (any_id, css_classes_and_warnings)| {
                    let CssClassesAndWarnings {
                        css_classes,
                        theme_warnings,
                    } = css_classes_and_warnings;

                    el_css_classes.insert(any_id, css_classes);
                    theme_warnings_acc.extend(theme_warnings.into_inner());

                    (el_css_classes, theme_warnings_acc)
                },
            )
    }

    fn node_tag_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
        tag_id: &'f TagId,
    ) -> impl Iterator<Item = (AnyId, CssClassesAndWarnings)> + 'f
    where
        T: Themeable,
    {
        let node_class_partials_defaults = self.get(&AnyIdOrDefaults::NodeDefaults);
        themeable.node_ids().filter_map(move |node_id| {
            let node_class_partials_specified = self.node_class_partials_specified(node_id);

            let any_id = Some(AnyId::from(node_id.clone()));
            let node_classes_and_warnings = CssClassMerger::node_tag_classes(
                node_class_partials_defaults,
                node_class_partials_specified,
                themeable,
                tag_id,
            );

            any_id.map(|any_id| (any_id, node_classes_and_warnings))
        })
    }

    fn edge_tag_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
        tag_id: &'f TagId,
    ) -> impl Iterator<Item = (AnyId, CssClassesAndWarnings)> + 'f
    where
        T: Themeable,
    {
        let edge_class_partials_defaults = self.get(&AnyIdOrDefaults::EdgeDefaults);

        themeable.edge_ids().filter_map(move |edge_id| {
            let edge_class_partials_specified = self.edge_class_partials_specified(edge_id);

            let any_id = Some(AnyId::from(edge_id.clone()));
            let edge_classes_and_warnings = CssClassMerger::edge_tag_classes(
                edge_class_partials_defaults,
                edge_class_partials_specified,
                themeable,
                tag_id,
            );

            any_id.map(|any_id| (any_id, edge_classes_and_warnings))
        })
    }
}

impl Deref for Theme {
    type Target = ThemeStyles;

    fn deref(&self) -> &Self::Target {
        &self.styles
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.styles
    }
}

impl From<ThemeStyles> for Theme {
    fn from(styles: ThemeStyles) -> Self {
        Self {
            merge_with_base: true,
            styles,
        }
    }
}
