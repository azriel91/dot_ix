use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

pub use self::{
    any_id_or_defaults::AnyIdOrDefaults, css_class_partials::CssClassPartials,
    css_classes::CssClasses, css_classes_builder::CssClassesBuilder, el_css_classes::ElCssClasses,
    highlight_state::HighlightState, theme_attr::ThemeAttr, themeable::Themeable,
    themeable_params::ThemeableParams,
};

mod any_id_or_defaults;
mod css_class_partials;
mod css_classes;
mod css_classes_builder;
mod el_css_classes;
mod highlight_state;
mod theme_attr;
mod themeable;
mod themeable_params;

/// Theme to style the generated diagram. Map of [`AnyIdOrDefaults`] to
/// [`CssClassPartials`].
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme(IndexMap<AnyIdOrDefaults, CssClassPartials>);

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
            node_defaults.insert(ThemeAttr::Padding, "1.5".into());
            node_defaults.insert(ThemeAttr::ShapeColor, "slate".into());

            node_defaults.insert(ThemeAttr::FillShadeNormal, "300".into());
            node_defaults.insert(ThemeAttr::FillShadeFocus, "200".into());
            node_defaults.insert(ThemeAttr::FillShadeHover, "100".into());
            node_defaults.insert(ThemeAttr::FillShadeActive, "200".into());

            node_defaults.insert(ThemeAttr::StrokeShadeNormal, "600".into());
            node_defaults.insert(ThemeAttr::StrokeShadeFocus, "500".into());
            node_defaults.insert(ThemeAttr::StrokeShadeHover, "400".into());
            node_defaults.insert(ThemeAttr::StrokeShadeActive, "300".into());

            node_defaults
        });

        theme.insert(AnyIdOrDefaults::EdgeDefaults, {
            let mut node_defaults = CssClassPartials::new();
            node_defaults.insert(ThemeAttr::ShapeColor, "slate".into());

            node_defaults.insert(ThemeAttr::FillShadeNormal, "800".into());
            node_defaults.insert(ThemeAttr::FillShadeFocus, "700".into());
            node_defaults.insert(ThemeAttr::FillShadeHover, "600".into());
            node_defaults.insert(ThemeAttr::FillShadeActive, "500".into());

            node_defaults.insert(ThemeAttr::StrokeShadeNormal, "950".into());
            node_defaults.insert(ThemeAttr::StrokeShadeFocus, "800".into());
            node_defaults.insert(ThemeAttr::StrokeShadeHover, "700".into());
            node_defaults.insert(ThemeAttr::StrokeShadeActive, "600".into());

            node_defaults
        });

        theme
    }

    /// Returns a new `Theme` with the given preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<AnyIdOrDefaults, CssClassPartials> {
        self.0
    }

    /// Computes the CSS utility classes for each element.
    ///
    /// The [`CssClasses`] produced will contain an entry for each node / edge
    /// ID from the themeable type.
    pub fn el_css_classes<T>(&self, themeable: &T) -> ElCssClasses
    where
        T: Themeable,
    {
        let node_class_partials_defaults = self.get(&AnyIdOrDefaults::NodeDefaults);

        themeable.node_ids().iter().map(|node_id| {
            let node_class_partials_specified = self.node_class_partials_specified(node_id);

            Self::node_classes(
                node_class_partials_defaults,
                node_class_partials_specified,
                themeable,
            )
        });

        todo!()
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

    fn node_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> Option<CssClasses>
    where
        T: Themeable,
    {
        match (defaults, specified) {
            (None, None) => None,
            (None, Some(node_class_partials)) | (Some(node_class_partials), None) => {
                let mut css_classes_builder = CssClassesBuilder::new();

                let themeable_node_stroke_classes =
                    |themeable: &dyn Themeable,
                     css_classes_builder: &mut CssClassesBuilder,
                     params: ThemeableParams<'_>| {
                        themeable.node_stroke_classes(css_classes_builder, params);
                    };

                let themeable_node_fill_classes =
                    |themeable: &dyn Themeable,
                     css_classes_builder: &mut CssClassesBuilder,
                     params: ThemeableParams<'_>| {
                        themeable.node_fill_classes(css_classes_builder, params);
                    };

                [
                    ColorParamGroupings::new(
                        HighlightState::Normal,
                        ThemeAttr::StrokeColorNormal,
                        ThemeAttr::StrokeColor,
                        ThemeAttr::StrokeShadeNormal,
                        ThemeAttr::StrokeShade,
                        themeable_node_stroke_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Focus,
                        ThemeAttr::StrokeColorFocus,
                        ThemeAttr::StrokeColor,
                        ThemeAttr::StrokeShadeFocus,
                        ThemeAttr::StrokeShade,
                        themeable_node_stroke_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Hover,
                        ThemeAttr::StrokeColorHover,
                        ThemeAttr::StrokeColor,
                        ThemeAttr::StrokeShadeHover,
                        ThemeAttr::StrokeShade,
                        themeable_node_stroke_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Active,
                        ThemeAttr::StrokeColorActive,
                        ThemeAttr::StrokeColor,
                        ThemeAttr::StrokeShadeActive,
                        ThemeAttr::StrokeShade,
                        themeable_node_stroke_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Normal,
                        ThemeAttr::FillColorNormal,
                        ThemeAttr::FillColor,
                        ThemeAttr::FillShadeNormal,
                        ThemeAttr::FillShade,
                        themeable_node_fill_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Focus,
                        ThemeAttr::FillColorFocus,
                        ThemeAttr::FillColor,
                        ThemeAttr::FillShadeFocus,
                        ThemeAttr::FillShade,
                        themeable_node_fill_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Hover,
                        ThemeAttr::FillColorHover,
                        ThemeAttr::FillColor,
                        ThemeAttr::FillShadeHover,
                        ThemeAttr::FillShade,
                        themeable_node_fill_classes,
                    ),
                    ColorParamGroupings::new(
                        HighlightState::Active,
                        ThemeAttr::FillColorActive,
                        ThemeAttr::FillColor,
                        ThemeAttr::FillShadeActive,
                        ThemeAttr::FillShade,
                        themeable_node_fill_classes,
                    ),
                ]
                .into_iter()
                .for_each(|css_classes_param_groupings| {
                    let ColorParamGroupings {
                        highlight_state,
                        color_key,
                        color_fallback_key,
                        shade_key,
                        shade_fallback_key,
                        fn_css_classes,
                    } = css_classes_param_groupings;

                    let fill_color = node_class_partials
                        .get(&color_key)
                        .or_else(|| node_class_partials.get(&color_fallback_key));
                    let fill_shade = node_class_partials
                        .get(&shade_key)
                        .or_else(|| node_class_partials.get(&shade_fallback_key));

                    fill_color
                        .zip(fill_shade)
                        .map(|(color, shade)| ThemeableParams {
                            highlight_state,
                            color,
                            shade,
                        })
                        .map(|params| fn_css_classes(themeable, &mut css_classes_builder, params));
                });

                [
                    SpacingParamGroupings::new(ThemeAttr::PaddingX, ThemeAttr::Padding),
                    SpacingParamGroupings::new(ThemeAttr::PaddingY, ThemeAttr::Padding),
                    SpacingParamGroupings::new(ThemeAttr::MarginX, ThemeAttr::Margin),
                    SpacingParamGroupings::new(ThemeAttr::MarginY, ThemeAttr::Margin),
                ]
                .into_iter()
                .for_each(|css_classes_param_groupings| {
                    let SpacingParamGroupings {
                        spacing_key,
                        spacing_fallback_key,
                    } = css_classes_param_groupings;

                    let spacing = node_class_partials
                        .get(&spacing_key)
                        .or_else(|| node_class_partials.get(&spacing_fallback_key));

                    spacing.map(|spacing| css_classes_builder.append(spacing));
                });

                if let Some(extra) = node_class_partials.get(&ThemeAttr::Extra) {
                    css_classes_builder.append(extra)
                }

                Some(css_classes_builder.build())
            }
            (Some(_), Some(_)) => todo!(),
        }
    }
}

/// Groupings of parameters to generate CSS classes for colour shades.
struct ColorParamGroupings {
    highlight_state: HighlightState,
    color_key: ThemeAttr,
    color_fallback_key: ThemeAttr,
    shade_key: ThemeAttr,
    shade_fallback_key: ThemeAttr,
    fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, ThemeableParams<'_>),
}

impl ColorParamGroupings {
    fn new(
        highlight_state: HighlightState,
        color_key: ThemeAttr,
        color_fallback_key: ThemeAttr,
        shade_key: ThemeAttr,
        shade_fallback_key: ThemeAttr,
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, ThemeableParams<'_>),
    ) -> Self {
        Self {
            highlight_state,
            color_key,
            color_fallback_key,
            shade_key,
            shade_fallback_key,
            fn_css_classes,
        }
    }
}

/// Groupings of parameters to generate CSS classes for spacing.
struct SpacingParamGroupings {
    spacing_key: ThemeAttr,
    spacing_fallback_key: ThemeAttr,
}

impl SpacingParamGroupings {
    fn new(spacing_key: ThemeAttr, spacing_fallback_key: ThemeAttr) -> Self {
        Self {
            spacing_key,
            spacing_fallback_key,
        }
    }
}

impl Deref for Theme {
    type Target = IndexMap<AnyIdOrDefaults, CssClassPartials>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexMap<AnyIdOrDefaults, CssClassPartials>> for Theme {
    fn from(inner: IndexMap<AnyIdOrDefaults, CssClassPartials>) -> Self {
        Self(inner)
    }
}
