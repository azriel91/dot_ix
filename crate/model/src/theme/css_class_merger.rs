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

        css_classes_builder.build()
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
