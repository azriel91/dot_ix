use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::{AnyId, EdgeId, NodeId};

pub use self::{
    any_id_or_defaults::AnyIdOrDefaults, color_params::ColorParams,
    css_class_partials::CssClassPartials, css_classes::CssClasses,
    css_classes_builder::CssClassesBuilder, el_css_classes::ElCssClasses,
    highlight_state::HighlightState, stroke_params::StrokeParams, theme_attr::ThemeAttr,
    themeable::Themeable,
};

mod any_id_or_defaults;
mod color_params;
mod css_class_partials;
mod css_classes;
mod css_classes_builder;
mod el_css_classes;
mod highlight_state;
mod stroke_params;
mod theme_attr;
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
    merge_with_base: bool,
    /// CSS utility class partials for each element.
    styles: IndexMap<AnyIdOrDefaults, CssClassPartials>,
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
            node_defaults.insert(ThemeAttr::OutlineStyle, "none".into());
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
            edge_defaults.insert(ThemeAttr::OutlineStyle, "none".into());
            edge_defaults.insert(ThemeAttr::OutlineStyleFocus, "dashed".into());

            edge_defaults
        });

        theme
    }

    /// Returns a new `Theme` with the given preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            merge_with_base: true,
            styles: IndexMap::with_capacity(capacity),
        }
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> IndexMap<AnyIdOrDefaults, CssClassPartials> {
        self.styles
    }

    /// Returns whether to merge with the base styles.
    pub fn merge_with_base(&self) -> bool {
        self.merge_with_base
    }

    /// Returns whether to merge with the base styles.
    pub fn merge_with_base_mut(&mut self) -> &mut bool {
        &mut self.merge_with_base
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
    pub fn el_css_classes<T>(&self, themeable: &T) -> ElCssClasses
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
            .collect()
    }

    fn node_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
    ) -> impl Iterator<Item = (AnyId, CssClasses)> + 'f
    where
        T: Themeable,
    {
        let node_class_partials_defaults = self.get(&AnyIdOrDefaults::NodeDefaults);
        themeable.node_ids().filter_map(move |node_id| {
            let node_class_partials_specified = self.node_class_partials_specified(node_id);

            let any_id = Some(AnyId::from(node_id.clone()));
            let node_classes = Self::node_classes(
                node_class_partials_defaults,
                node_class_partials_specified,
                themeable,
            );

            any_id.zip(node_classes)
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

    fn node_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> Option<CssClasses>
    where
        T: Themeable,
    {
        let mut css_classes_builder = CssClassesBuilder::new();

        let themeable_node_outline_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: StrokeParams<'_>| {
                themeable.node_outline_classes(css_classes_builder, params);
            };

        let themeable_node_stroke_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: StrokeParams<'_>| {
                themeable.node_stroke_classes(css_classes_builder, params);
            };

        let themeable_node_fill_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: ColorParams<'_>| {
                themeable.node_fill_classes(css_classes_builder, params);
            };

        Self::outline_classes(
            themeable_node_outline_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );
        Self::stroke_classes(
            themeable_node_stroke_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );
        Self::fill_classes(
            themeable_node_fill_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );

        [
            SpacingParamGroupings::new("px", &[ThemeAttr::PaddingX, ThemeAttr::Padding]),
            SpacingParamGroupings::new("py", &[ThemeAttr::PaddingY, ThemeAttr::Padding]),
            SpacingParamGroupings::new("mx", &[ThemeAttr::MarginX, ThemeAttr::Margin]),
            SpacingParamGroupings::new("my", &[ThemeAttr::MarginY, ThemeAttr::Margin]),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            let SpacingParamGroupings {
                spacing_prefix,
                spacing_keys,
            } = css_classes_param_groupings;

            let spacing = attr_value_find(spacing_keys, specified, defaults);

            spacing
                .map(|spacing| css_classes_builder.append(&format!("{spacing_prefix}-{spacing}")));
        });

        Self::cursor_classes(specified, defaults, &mut css_classes_builder);
        Self::extra_classes(specified, defaults, &mut css_classes_builder);

        Some(css_classes_builder.build())
    }

    fn edge_el_css_classes<'f, T>(
        &'f self,
        themeable: &'f T,
    ) -> impl Iterator<Item = (AnyId, CssClasses)> + 'f
    where
        T: Themeable,
    {
        let edge_class_partials_defaults = self.get(&AnyIdOrDefaults::EdgeDefaults);

        themeable.edge_ids().filter_map(move |edge_id| {
            let edge_class_partials_specified = self.edge_class_partials_specified(edge_id);

            let any_id = Some(AnyId::from(edge_id.clone()));
            let edge_classes = Self::edge_classes(
                edge_class_partials_defaults,
                edge_class_partials_specified,
                themeable,
            );

            any_id.zip(edge_classes)
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

    fn edge_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> Option<CssClasses>
    where
        T: Themeable,
    {
        let mut css_classes_builder = CssClassesBuilder::new();

        let themeable_edge_outline_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: StrokeParams<'_>| {
                themeable.edge_outline_classes(css_classes_builder, params);
            };

        let themeable_edge_stroke_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: StrokeParams<'_>| {
                themeable.edge_stroke_classes(css_classes_builder, params);
            };

        let themeable_edge_fill_classes =
            |themeable: &dyn Themeable,
             css_classes_builder: &mut CssClassesBuilder,
             params: ColorParams<'_>| {
                themeable.edge_fill_classes(css_classes_builder, params);
            };

        Self::outline_classes(
            themeable_edge_outline_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );
        Self::stroke_classes(
            themeable_edge_stroke_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );
        Self::fill_classes(
            themeable_edge_fill_classes,
            specified,
            defaults,
            themeable,
            &mut css_classes_builder,
        );

        Self::cursor_classes(specified, defaults, &mut css_classes_builder);
        Self::extra_classes(specified, defaults, &mut css_classes_builder);

        Some(css_classes_builder.build())
    }

    fn outline_classes(
        fn_outline_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
        [
            StrokeParamGroupings::new(
                HighlightState::Normal,
                &[ThemeAttr::OutlineColorNormal, ThemeAttr::OutlineColor],
                &[ThemeAttr::OutlineShadeNormal, ThemeAttr::OutlineShade],
                &[ThemeAttr::OutlineStyleNormal, ThemeAttr::OutlineStyle],
                fn_outline_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Focus,
                &[ThemeAttr::OutlineColorFocus, ThemeAttr::OutlineColor],
                &[ThemeAttr::OutlineShadeFocus, ThemeAttr::OutlineShade],
                &[ThemeAttr::OutlineStyleFocus, ThemeAttr::OutlineStyle],
                fn_outline_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Hover,
                &[ThemeAttr::OutlineColorHover, ThemeAttr::OutlineColor],
                &[ThemeAttr::OutlineShadeHover, ThemeAttr::OutlineShade],
                &[ThemeAttr::OutlineStyleHover, ThemeAttr::OutlineStyle],
                fn_outline_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Active,
                &[ThemeAttr::OutlineColorActive, ThemeAttr::OutlineColor],
                &[ThemeAttr::OutlineShadeActive, ThemeAttr::OutlineShade],
                &[ThemeAttr::OutlineStyleActive, ThemeAttr::OutlineStyle],
                fn_outline_classes,
            ),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            let StrokeParamGroupings {
                highlight_state,
                color_keys,
                shade_keys,
                stroke_style_keys,
                fn_css_classes,
            } = css_classes_param_groupings;

            let color = attr_value_find(color_keys, specified, defaults);
            let shade = attr_value_find(shade_keys, specified, defaults);
            let outline_style = attr_value_find(stroke_style_keys, specified, defaults);

            let outline_width = specified
                .and_then(|partials| partials.get(&ThemeAttr::OutlineWidth))
                .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::OutlineWidth)));

            if let Some(params) = color.zip(shade).zip(outline_width).zip(outline_style).map(
                |(((color, shade), outline_width), outline_style)| StrokeParams {
                    color_params: ColorParams {
                        highlight_state,
                        color,
                        shade,
                    },
                    stroke_width: outline_width,
                    stroke_style: outline_style,
                },
            ) {
                fn_css_classes(themeable, css_classes_builder, params)
            }
        });
    }

    fn stroke_classes(
        fn_stroke_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
        [
            StrokeParamGroupings::new(
                HighlightState::Normal,
                &[
                    ThemeAttr::StrokeColorNormal,
                    ThemeAttr::StrokeColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::StrokeShadeNormal, ThemeAttr::StrokeShade],
                &[ThemeAttr::StrokeStyleNormal, ThemeAttr::StrokeStyle],
                fn_stroke_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Focus,
                &[
                    ThemeAttr::StrokeColorFocus,
                    ThemeAttr::StrokeColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::StrokeShadeFocus, ThemeAttr::StrokeShade],
                &[ThemeAttr::StrokeStyleFocus, ThemeAttr::StrokeStyle],
                fn_stroke_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::FocusHover,
                &[
                    ThemeAttr::StrokeColorHover,
                    ThemeAttr::StrokeColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::StrokeShadeHover, ThemeAttr::StrokeShade],
                &[ThemeAttr::StrokeStyleHover, ThemeAttr::StrokeStyle],
                fn_stroke_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Hover,
                &[
                    ThemeAttr::StrokeColorHover,
                    ThemeAttr::StrokeColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::StrokeShadeHover, ThemeAttr::StrokeShade],
                &[ThemeAttr::StrokeStyleHover, ThemeAttr::StrokeStyle],
                fn_stroke_classes,
            ),
            StrokeParamGroupings::new(
                HighlightState::Active,
                &[
                    ThemeAttr::StrokeColorActive,
                    ThemeAttr::StrokeColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::StrokeShadeActive, ThemeAttr::StrokeShade],
                &[ThemeAttr::StrokeStyleActive, ThemeAttr::StrokeStyle],
                fn_stroke_classes,
            ),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            let StrokeParamGroupings {
                highlight_state,
                color_keys,
                shade_keys,
                stroke_style_keys,
                fn_css_classes,
            } = css_classes_param_groupings;

            let color = attr_value_find(color_keys, specified, defaults);
            let shade = attr_value_find(shade_keys, specified, defaults);
            let stroke_style = attr_value_find(stroke_style_keys, specified, defaults);

            let stroke_width = specified
                .and_then(|partials| partials.get(&ThemeAttr::StrokeWidth))
                .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::StrokeWidth)));

            if let Some(params) = color.zip(shade).zip(stroke_width).zip(stroke_style).map(
                |(((color, shade), stroke_width), stroke_style)| StrokeParams {
                    color_params: ColorParams {
                        highlight_state,
                        color,
                        shade,
                    },
                    stroke_width,
                    stroke_style,
                },
            ) {
                fn_css_classes(themeable, css_classes_builder, params)
            }
        });
    }

    fn fill_classes(
        fn_fill_classes: fn(&dyn Themeable, &mut CssClassesBuilder, ColorParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
        [
            ColorParamGroupings::new(
                HighlightState::Normal,
                &[
                    ThemeAttr::FillColorNormal,
                    ThemeAttr::FillColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::FillShadeNormal, ThemeAttr::FillShade],
                fn_fill_classes,
            ),
            ColorParamGroupings::new(
                HighlightState::Focus,
                &[
                    ThemeAttr::FillColorFocus,
                    ThemeAttr::FillColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::FillShadeFocus, ThemeAttr::FillShade],
                fn_fill_classes,
            ),
            ColorParamGroupings::new(
                HighlightState::FocusHover,
                &[
                    ThemeAttr::FillColorHover,
                    ThemeAttr::FillColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::FillShadeHover, ThemeAttr::FillShade],
                fn_fill_classes,
            ),
            ColorParamGroupings::new(
                HighlightState::Hover,
                &[
                    ThemeAttr::FillColorHover,
                    ThemeAttr::FillColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::FillShadeHover, ThemeAttr::FillShade],
                fn_fill_classes,
            ),
            ColorParamGroupings::new(
                HighlightState::Active,
                &[
                    ThemeAttr::FillColorActive,
                    ThemeAttr::FillColor,
                    ThemeAttr::ShapeColor,
                ],
                &[ThemeAttr::FillShadeActive, ThemeAttr::FillShade],
                fn_fill_classes,
            ),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            let ColorParamGroupings {
                highlight_state,
                color_keys,
                shade_keys,
                fn_css_classes,
            } = css_classes_param_groupings;

            let color = attr_value_find(color_keys, specified, defaults);
            let shade = attr_value_find(shade_keys, specified, defaults);

            if let Some(params) = color.zip(shade).map(|(color, shade)| ColorParams {
                highlight_state,
                color,
                shade,
            }) {
                fn_css_classes(themeable, css_classes_builder, params)
            }
        });
    }

    fn cursor_classes(
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
        specified
            .and_then(|partials| partials.get(&ThemeAttr::Cursor))
            .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::Cursor)))
            .map(|cursor| css_classes_builder.append(&format!("cursor-{cursor}")));
    }

    fn extra_classes(
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
        specified
            .and_then(|partials| partials.get(&ThemeAttr::Extra))
            .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::Extra)))
            .map(|extra| css_classes_builder.append(extra));
    }
}

/// Finds an attributes with multiple levels of fallbacks.
///
/// The current algorithm:
///
/// 1. Most specific key from the element's class partials.
/// 2. Most specific key from the element (node / edge) defaults.
/// 3. Less specific key from the element's class partials.
/// 4. Less specific key from the element defaults.
///
/// However, this is surprising when the base theme provides a specific default,
/// and the user's less specific default is not used because the base theme's
/// more specific default is used.
fn attr_value_find<'attr>(
    attr_keys: &'attr [ThemeAttr],
    el_class_partials: Option<&'attr CssClassPartials>,
    defaults: Option<&'attr CssClassPartials>,
) -> Option<&'attr str> {
    attr_keys
        .iter()
        .find_map(|attr_key| {
            el_class_partials
                .and_then(|partials| partials.get(attr_key))
                .or_else(|| defaults.and_then(|partials| partials.get(attr_key)))
        })
        .map(String::as_str)
}

/// Groupings of parameters to generate CSS classes for colour shades.
struct ColorParamGroupings<Params> {
    highlight_state: HighlightState,
    /// List of keys to fallback on.
    ///
    /// State specific color, state agnostic color, shape color.
    color_keys: &'static [ThemeAttr],
    shade_keys: &'static [ThemeAttr],
    fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
}

impl<Params> ColorParamGroupings<Params> {
    fn new(
        highlight_state: HighlightState,
        color_keys: &'static [ThemeAttr],
        shade_keys: &'static [ThemeAttr],
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state,
            color_keys,
            shade_keys,
            fn_css_classes,
        }
    }
}

/// Groupings of parameters to generate CSS classes for colour shades.
struct StrokeParamGroupings<Params> {
    highlight_state: HighlightState,
    /// List of keys to fallback on.
    ///
    /// State specific color, state agnostic color, shape color.
    color_keys: &'static [ThemeAttr],
    shade_keys: &'static [ThemeAttr],
    stroke_style_keys: &'static [ThemeAttr],
    fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
}

impl<Params> StrokeParamGroupings<Params> {
    fn new(
        highlight_state: HighlightState,
        color_keys: &'static [ThemeAttr],
        shade_keys: &'static [ThemeAttr],
        stroke_style_keys: &'static [ThemeAttr],
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state,
            color_keys,
            shade_keys,
            stroke_style_keys,
            fn_css_classes,
        }
    }
}

/// Groupings of parameters to generate CSS classes for spacing.
struct SpacingParamGroupings {
    spacing_prefix: &'static str,
    spacing_keys: &'static [ThemeAttr],
}

impl SpacingParamGroupings {
    fn new(spacing_prefix: &'static str, spacing_keys: &'static [ThemeAttr]) -> Self {
        Self {
            spacing_prefix,
            spacing_keys,
        }
    }
}

impl Deref for Theme {
    type Target = IndexMap<AnyIdOrDefaults, CssClassPartials>;

    fn deref(&self) -> &Self::Target {
        &self.styles
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.styles
    }
}

impl From<IndexMap<AnyIdOrDefaults, CssClassPartials>> for Theme {
    fn from(styles: IndexMap<AnyIdOrDefaults, CssClassPartials>) -> Self {
        Self {
            merge_with_base: true,
            styles,
        }
    }
}
