use crate::theme::{
    ColorParams, CssClassPartials, CssClasses, CssClassesBuilder, HighlightState, StrokeParams,
    ThemeAttr, Themeable,
};

/// Common logic for merging [`CssClassPartials`] into [`CssClasses`].
pub struct CssClassMerger;

impl CssClassMerger {
    /// Returns the CSS classes for a node in a particular themeable rendering.
    ///
    /// This merges the specified themed values over the defaults.
    pub fn node_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> CssClasses
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
            &mut css_classes_builder,
            themeable_node_outline_classes,
            specified,
            defaults,
            themeable,
        );
        Self::stroke_classes_append(
            &mut css_classes_builder,
            themeable_node_stroke_classes,
            specified,
            defaults,
            themeable,
        );
        Self::fill_classes_append(
            &mut css_classes_builder,
            themeable_node_fill_classes,
            specified,
            defaults,
            themeable,
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

        css_classes_builder.build()
    }

    /// Returns the CSS classes for an edge in a particular themeable rendering.
    ///
    /// This merges the specified themed values over the defaults.
    pub fn edge_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> CssClasses
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
            &mut css_classes_builder,
            themeable_edge_outline_classes,
            specified,
            defaults,
            themeable,
        );
        Self::stroke_classes_append(
            &mut css_classes_builder,
            themeable_edge_stroke_classes,
            specified,
            defaults,
            themeable,
        );
        Self::fill_classes_append(
            &mut css_classes_builder,
            themeable_edge_fill_classes,
            specified,
            defaults,
            themeable,
        );

        Self::cursor_classes(specified, defaults, &mut css_classes_builder);
        Self::extra_classes(specified, defaults, &mut css_classes_builder);

        css_classes_builder.build()
    }

    fn outline_classes(
        css_classes_builder: &mut CssClassesBuilder,
        fn_outline_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        [
            StrokeParamGroupings::new_outline_normal(fn_outline_classes),
            StrokeParamGroupings::new_outline_focus(fn_outline_classes),
            StrokeParamGroupings::new_outline_hover(fn_outline_classes),
            StrokeParamGroupings::new_outline_active(fn_outline_classes),
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

    /// Appends CSS classes for stroke styling for all [`HighlightState`]s to
    /// the CSS classes builder.
    fn stroke_classes_append(
        css_classes_builder: &mut CssClassesBuilder,
        fn_stroke_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        [
            StrokeParamGroupings::new_stroke_normal(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_focus(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_focus_hover(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_hover(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_active(fn_stroke_classes),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            Self::stroke_classes_highlight_state_append(
                css_classes_builder,
                &css_classes_param_groupings,
                specified,
                defaults,
                themeable,
            );
        });
    }

    /// Appends CSS classes for stroke styling for a given [`HighlightState`] to
    /// the CSS classes builder.
    fn stroke_classes_highlight_state_append<'f1, 'f2: 'f1>(
        css_classes_builder: &mut CssClassesBuilder,
        css_classes_param_groupings: &StrokeParamGroupings<StrokeParams<'f1>>,
        specified: Option<&'f2 CssClassPartials>,
        defaults: Option<&'f2 CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
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
                    highlight_state: *highlight_state,
                    color,
                    shade,
                },
                stroke_width,
                stroke_style,
            },
        ) {
            fn_css_classes(themeable, css_classes_builder, params)
        }
    }

    /// Appends CSS classes for fill styling for all [`HighlightState`]s to
    /// the CSS classes builder.
    fn fill_classes_append(
        css_classes_builder: &mut CssClassesBuilder,
        fn_fill_classes: fn(&dyn Themeable, &mut CssClassesBuilder, ColorParams<'_>),
        specified: Option<&CssClassPartials>,
        defaults: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        [
            ColorParamGroupings::new_fill_normal(fn_fill_classes),
            ColorParamGroupings::new_fill_focus(fn_fill_classes),
            ColorParamGroupings::new_fill_focus_hover(fn_fill_classes),
            ColorParamGroupings::new_fill_hover(fn_fill_classes),
            ColorParamGroupings::new_fill_active(fn_fill_classes),
        ]
        .into_iter()
        .for_each(|css_classes_param_groupings| {
            Self::fill_classes_highlight_state_append(
                css_classes_param_groupings,
                specified,
                defaults,
                themeable,
                css_classes_builder,
            );
        });
    }

    /// Appends CSS classes for fill styling for a given [`HighlightState`]s to
    /// the CSS classes builder.
    fn fill_classes_highlight_state_append<'f1, 'f2: 'f1>(
        css_classes_param_groupings: ColorParamGroupings<ColorParams<'f1>>,
        specified: Option<&'f2 CssClassPartials>,
        defaults: Option<&'f2 CssClassPartials>,
        themeable: &dyn Themeable,
        css_classes_builder: &mut CssClassesBuilder,
    ) {
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
    fn new_fill_normal(fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params)) -> Self {
        Self {
            highlight_state: HighlightState::Normal,
            color_keys: &[
                ThemeAttr::FillColorNormal,
                ThemeAttr::FillColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::FillShadeNormal, ThemeAttr::FillShade],
            fn_css_classes,
        }
    }

    fn new_fill_focus(fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params)) -> Self {
        Self {
            highlight_state: HighlightState::Focus,
            color_keys: &[
                ThemeAttr::FillColorFocus,
                ThemeAttr::FillColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::FillShadeFocus, ThemeAttr::FillShade],
            fn_css_classes,
        }
    }

    fn new_fill_focus_hover(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::FocusHover,
            color_keys: &[
                ThemeAttr::FillColorHover,
                ThemeAttr::FillColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::FillShadeHover, ThemeAttr::FillShade],
            fn_css_classes,
        }
    }

    fn new_fill_hover(fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params)) -> Self {
        Self {
            highlight_state: HighlightState::Hover,
            color_keys: &[
                ThemeAttr::FillColorHover,
                ThemeAttr::FillColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::FillShadeHover, ThemeAttr::FillShade],
            fn_css_classes,
        }
    }

    fn new_fill_active(fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params)) -> Self {
        Self {
            highlight_state: HighlightState::Active,
            color_keys: &[
                ThemeAttr::FillColorActive,
                ThemeAttr::FillColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::FillShadeActive, ThemeAttr::FillShade],
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
    fn new_stroke_normal(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Normal,
            color_keys: &[
                ThemeAttr::StrokeColorNormal,
                ThemeAttr::StrokeColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::StrokeShadeNormal, ThemeAttr::StrokeShade],
            stroke_style_keys: &[ThemeAttr::StrokeStyleNormal, ThemeAttr::StrokeStyle],
            fn_css_classes,
        }
    }

    fn new_stroke_focus(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Focus,
            color_keys: &[
                ThemeAttr::StrokeColorFocus,
                ThemeAttr::StrokeColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::StrokeShadeFocus, ThemeAttr::StrokeShade],
            stroke_style_keys: &[ThemeAttr::StrokeStyleFocus, ThemeAttr::StrokeStyle],
            fn_css_classes,
        }
    }

    fn new_stroke_focus_hover(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::FocusHover,
            color_keys: &[
                ThemeAttr::StrokeColorHover,
                ThemeAttr::StrokeColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::StrokeShadeHover, ThemeAttr::StrokeShade],
            stroke_style_keys: &[ThemeAttr::StrokeStyleHover, ThemeAttr::StrokeStyle],
            fn_css_classes,
        }
    }

    fn new_stroke_hover(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Hover,
            color_keys: &[
                ThemeAttr::StrokeColorHover,
                ThemeAttr::StrokeColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::StrokeShadeHover, ThemeAttr::StrokeShade],
            stroke_style_keys: &[ThemeAttr::StrokeStyleHover, ThemeAttr::StrokeStyle],
            fn_css_classes,
        }
    }

    fn new_stroke_active(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Active,
            color_keys: &[
                ThemeAttr::StrokeColorActive,
                ThemeAttr::StrokeColor,
                ThemeAttr::ShapeColor,
            ],
            shade_keys: &[ThemeAttr::StrokeShadeActive, ThemeAttr::StrokeShade],
            stroke_style_keys: &[ThemeAttr::StrokeStyleActive, ThemeAttr::StrokeStyle],
            fn_css_classes,
        }
    }

    fn new_outline_normal(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Normal,
            color_keys: &[ThemeAttr::OutlineColorNormal, ThemeAttr::OutlineColor],
            shade_keys: &[ThemeAttr::OutlineShadeNormal, ThemeAttr::OutlineShade],
            stroke_style_keys: &[ThemeAttr::OutlineStyleNormal, ThemeAttr::OutlineStyle],
            fn_css_classes,
        }
    }

    fn new_outline_focus(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Focus,
            color_keys: &[ThemeAttr::OutlineColorFocus, ThemeAttr::OutlineColor],
            shade_keys: &[ThemeAttr::OutlineShadeFocus, ThemeAttr::OutlineShade],
            stroke_style_keys: &[ThemeAttr::OutlineStyleFocus, ThemeAttr::OutlineStyle],
            fn_css_classes,
        }
    }

    fn new_outline_hover(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Hover,
            color_keys: &[ThemeAttr::OutlineColorHover, ThemeAttr::OutlineColor],
            shade_keys: &[ThemeAttr::OutlineShadeHover, ThemeAttr::OutlineShade],
            stroke_style_keys: &[ThemeAttr::OutlineStyleHover, ThemeAttr::OutlineStyle],
            fn_css_classes,
        }
    }

    fn new_outline_active(
        fn_css_classes: fn(&dyn Themeable, &mut CssClassesBuilder, Params),
    ) -> Self {
        Self {
            highlight_state: HighlightState::Active,
            color_keys: &[ThemeAttr::OutlineColorActive, ThemeAttr::OutlineColor],
            shade_keys: &[ThemeAttr::OutlineShadeActive, ThemeAttr::OutlineShade],
            stroke_style_keys: &[ThemeAttr::OutlineStyleActive, ThemeAttr::OutlineStyle],
            fn_css_classes,
        }
    }
}
