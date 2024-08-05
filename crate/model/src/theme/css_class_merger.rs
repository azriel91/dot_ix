use crate::{
    common::TagId,
    theme::{
        ColorParams, CssClassPartials, CssClassesAndWarnings, CssClassesBuilder, HighlightState,
        StrokeParams, StyleFor, ThemeAttr, ThemeWarnings, Themeable,
    },
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
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        Self::node_classes_calculate(defaults, specified, themeable, StyleFor::Regular)
    }

    /// Returns the CSS classes for a node associated with a tag.
    pub fn node_tag_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
        tag_id: &TagId,
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        Self::node_classes_calculate(defaults, specified, themeable, StyleFor::TagFocus(tag_id))
    }

    /// Returns the CSS classes for a node in a particular themeable rendering.
    ///
    /// This merges the specified themed values over the defaults.
    fn node_classes_calculate<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
        style_for: StyleFor<'_>,
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        let mut css_classes_builder = CssClassesBuilder::new(style_for);
        let mut warnings = ThemeWarnings::new();

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

        Self::outline_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_node_outline_classes,
            defaults,
            specified,
            themeable,
        );
        Self::stroke_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_node_stroke_classes,
            defaults,
            specified,
            themeable,
        );
        Self::fill_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_node_fill_classes,
            defaults,
            specified,
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

            let spacing = attr_value_find(spacing_keys, defaults, specified);

            spacing.map(|(_spacing_attr, spacing)| {
                css_classes_builder.append(&format!("{spacing_prefix}-{spacing}"))
            });
        });

        Self::visibility_classes(&mut css_classes_builder, defaults, specified);
        Self::cursor_classes(&mut css_classes_builder, defaults, specified);
        Self::extra_classes(&mut css_classes_builder, defaults, specified);

        let css_classes = css_classes_builder.build();

        CssClassesAndWarnings::new(css_classes, warnings)
    }

    /// Returns the CSS classes for an edge in a particular themeable rendering.
    ///
    /// This merges the specified themed values over the defaults.
    pub fn edge_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        Self::edge_classes_calculate(defaults, specified, themeable, StyleFor::Regular)
    }

    /// Returns the CSS classes for an edge associated with a tag.
    pub fn edge_tag_classes<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
        tag_id: &TagId,
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        Self::edge_classes_calculate(defaults, specified, themeable, StyleFor::TagFocus(tag_id))
    }

    /// Returns the CSS classes for an edge in a particular themeable rendering.
    ///
    /// This merges the specified themed values over the defaults.
    fn edge_classes_calculate<T>(
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &T,
        style_for: StyleFor<'_>,
    ) -> CssClassesAndWarnings
    where
        T: Themeable,
    {
        let mut css_classes_builder = CssClassesBuilder::new(style_for);
        let mut warnings = ThemeWarnings::new();

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

        Self::outline_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_edge_outline_classes,
            defaults,
            specified,
            themeable,
        );
        Self::stroke_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_edge_stroke_classes,
            defaults,
            specified,
            themeable,
        );
        Self::fill_classes_append(
            &mut css_classes_builder,
            &mut warnings,
            themeable_edge_fill_classes,
            defaults,
            specified,
            themeable,
        );

        Self::visibility_classes(&mut css_classes_builder, defaults, specified);
        Self::cursor_classes(&mut css_classes_builder, defaults, specified);
        Self::extra_classes(&mut css_classes_builder, defaults, specified);

        let css_classes = css_classes_builder.build();

        CssClassesAndWarnings::new(css_classes, warnings)
    }

    fn outline_classes_append(
        css_classes_builder: &mut CssClassesBuilder,
        warnings: &mut ThemeWarnings,
        fn_outline_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        let stroke_class_append_result = [
            StrokeParamGroupings::new_outline_normal(fn_outline_classes),
            StrokeParamGroupings::new_outline_focus(fn_outline_classes),
            StrokeParamGroupings::new_outline_hover(fn_outline_classes),
            StrokeParamGroupings::new_outline_active(fn_outline_classes),
        ]
        .into_iter()
        .fold(
            Vec::new(),
            |mut stroke_class_append_result_acc, css_classes_param_groupings| {
                let stroke_class_append_result = Self::stroke_classes_highlight_state_append(
                    css_classes_builder,
                    &css_classes_param_groupings,
                    defaults,
                    specified,
                    themeable,
                );

                stroke_class_append_result_acc.push(stroke_class_append_result);

                stroke_class_append_result_acc
            },
        );

        if !stroke_class_append_result
            .iter()
            .any(|stroke_class_append_result| {
                matches!(stroke_class_append_result, StrokeClassAppendResult::Added)
            })
        {
            stroke_class_append_result
                .into_iter()
                .for_each(|stroke_class_append_result| {
                    if let StrokeClassAppendResult::NoChange {
                        style,
                        width,
                        color,
                        shade,
                    } = stroke_class_append_result
                    {
                        warnings.insert(format!(
                            "Outline attributes partially specified, \
                            so outline classes will not be applied. \
                            outline_style: `{style}`, outline_width: `{width}`, \
                            outline_color: `{color}`, outline_shade: `{shade}`.",
                            style = style
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            width = width
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            color = color
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            shade = shade
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                        ));
                    }
                });
        }
    }

    /// Appends CSS classes for stroke styling for all [`HighlightState`]s to
    /// the CSS classes builder.
    fn stroke_classes_append(
        css_classes_builder: &mut CssClassesBuilder,
        warnings: &mut ThemeWarnings,
        fn_stroke_classes: fn(&dyn Themeable, &mut CssClassesBuilder, StrokeParams<'_>),
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        let stroke_class_append_result = [
            StrokeParamGroupings::new_stroke_normal(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_focus(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_focus_hover(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_hover(fn_stroke_classes),
            StrokeParamGroupings::new_stroke_active(fn_stroke_classes),
        ]
        .into_iter()
        .fold(
            Vec::new(),
            |mut stroke_class_append_result_acc, css_classes_param_groupings| {
                let stroke_class_append_result = Self::stroke_classes_highlight_state_append(
                    css_classes_builder,
                    &css_classes_param_groupings,
                    defaults,
                    specified,
                    themeable,
                );

                stroke_class_append_result_acc.push(stroke_class_append_result);

                stroke_class_append_result_acc
            },
        );

        if !stroke_class_append_result
            .iter()
            .any(|stroke_class_append_result| {
                matches!(stroke_class_append_result, StrokeClassAppendResult::Added)
            })
        {
            stroke_class_append_result
                .into_iter()
                .for_each(|stroke_class_append_result| {
                    if let StrokeClassAppendResult::NoChange {
                        style,
                        width,
                        color,
                        shade,
                    } = stroke_class_append_result
                    {
                        warnings.insert(format!(
                            "Stroke attributes partially specified, \
                                so stroke classes will not be applied. \
                                stroke_style: `{style}`, stroke_width: `{width}`, \
                                stroke_color: `{color}`, stroke_shade: `{shade}`.",
                            style = style
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            width = width
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            color = color
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            shade = shade
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                        ));
                    }
                });
        }
    }

    /// Appends CSS classes for stroke styling for a given [`HighlightState`] to
    /// the CSS classes builder.
    fn stroke_classes_highlight_state_append<'f1, 'f2: 'f1>(
        css_classes_builder: &mut CssClassesBuilder,
        css_classes_param_groupings: &StrokeParamGroupings<StrokeParams<'f1>>,
        defaults: Option<&'f2 CssClassPartials>,
        specified: Option<&'f2 CssClassPartials>,
        themeable: &dyn Themeable,
    ) -> StrokeClassAppendResult<'f2> {
        let StrokeParamGroupings {
            highlight_state,
            color_keys,
            shade_keys,
            stroke_style_keys,
            stroke_width_keys,
            fn_css_classes,
        } = css_classes_param_groupings;

        let stroke_style = attr_value_find(stroke_style_keys, defaults, specified);
        let stroke_width = attr_value_find(stroke_width_keys, defaults, specified);
        let color = attr_value_find(color_keys, defaults, specified);
        let shade = attr_value_find(shade_keys, defaults, specified);

        match (stroke_style, stroke_width, color, shade) {
            (None, None, None, None) => StrokeClassAppendResult::NoAttrsSpecified,
            (
                Some((_stroke_style_attr, stroke_style)),
                Some((_stroke_width_attr, stroke_width)),
                Some((_color_attr, color)),
                Some((_shade_attr, shade)),
            ) => {
                let params = StrokeParams {
                    color_params: ColorParams {
                        highlight_state: *highlight_state,
                        color,
                        shade,
                    },
                    stroke_width,
                    stroke_style,
                };

                fn_css_classes(themeable, css_classes_builder, params);
                StrokeClassAppendResult::Added
            }
            (style, width, color, shade) => StrokeClassAppendResult::NoChange {
                style,
                width,
                color,
                shade,
            },
        }
    }

    /// Appends CSS classes for fill styling for all [`HighlightState`]s to
    /// the CSS classes builder.
    fn fill_classes_append(
        css_classes_builder: &mut CssClassesBuilder,
        warnings: &mut ThemeWarnings,
        fn_fill_classes: fn(&dyn Themeable, &mut CssClassesBuilder, ColorParams<'_>),
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
        themeable: &dyn Themeable,
    ) {
        let fill_class_append_result = [
            ColorParamGroupings::new_fill_normal(fn_fill_classes),
            ColorParamGroupings::new_fill_focus(fn_fill_classes),
            ColorParamGroupings::new_fill_focus_hover(fn_fill_classes),
            ColorParamGroupings::new_fill_hover(fn_fill_classes),
            ColorParamGroupings::new_fill_active(fn_fill_classes),
        ]
        .into_iter()
        .fold(
            Vec::new(),
            |mut fill_class_append_result_acc, css_classes_param_groupings| {
                let fill_class_append_result = Self::fill_classes_highlight_state_append(
                    css_classes_param_groupings,
                    defaults,
                    specified,
                    themeable,
                    css_classes_builder,
                );

                fill_class_append_result_acc.push(fill_class_append_result);

                fill_class_append_result_acc
            },
        );

        if !fill_class_append_result
            .iter()
            .any(|fill_class_append_result| {
                matches!(fill_class_append_result, FillClassAppendResult::Added)
            })
        {
            fill_class_append_result
                .into_iter()
                .for_each(|fill_class_append_result| {
                    if let FillClassAppendResult::NoChange { color, shade } =
                        fill_class_append_result
                    {
                        warnings.insert(format!(
                            "Fill attributes partially specified, \
                            so fill classes will not be applied. \
                            fill_color: `{color_value}`, fill_shade: `{shade_value}`.",
                            color_value = color
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                            shade_value = shade
                                .map(|(_attr, value)| value.to_string())
                                .as_deref()
                                .unwrap_or("<none>"),
                        ));
                    }
                });
        }
    }

    /// Appends CSS classes for fill styling for a given [`HighlightState`]s to
    /// the CSS classes builder.
    fn fill_classes_highlight_state_append<'f1, 'f2: 'f1>(
        css_classes_param_groupings: ColorParamGroupings<ColorParams<'f1>>,
        defaults: Option<&'f2 CssClassPartials>,
        specified: Option<&'f2 CssClassPartials>,
        themeable: &dyn Themeable,
        css_classes_builder: &mut CssClassesBuilder,
    ) -> FillClassAppendResult<'f2> {
        let ColorParamGroupings {
            highlight_state,
            color_keys,
            shade_keys,
            fn_css_classes,
        } = css_classes_param_groupings;

        let color = attr_value_find(color_keys, defaults, specified);
        let shade = attr_value_find(shade_keys, defaults, specified);

        match (color, shade) {
            (None, None) => FillClassAppendResult::NoAttrsSpecified,
            (Some((_color_attr, color)), Some((_shade_attr, shade))) => {
                let params = ColorParams {
                    highlight_state,
                    color,
                    shade,
                };
                fn_css_classes(themeable, css_classes_builder, params);
                FillClassAppendResult::Added
            }
            (color, shade) => FillClassAppendResult::NoChange { color, shade },
        }
    }

    fn visibility_classes(
        css_classes_builder: &mut CssClassesBuilder,
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
    ) {
        specified
            .and_then(|partials| partials.get(&ThemeAttr::Visibility))
            .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::Visibility)))
            .map(|visibility| css_classes_builder.append(visibility));
    }

    fn cursor_classes(
        css_classes_builder: &mut CssClassesBuilder,
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
    ) {
        specified
            .and_then(|partials| partials.get(&ThemeAttr::Cursor))
            .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::Cursor)))
            .map(|cursor| css_classes_builder.append(&format!("cursor-{cursor}")));
    }

    fn extra_classes(
        css_classes_builder: &mut CssClassesBuilder,
        defaults: Option<&CssClassPartials>,
        specified: Option<&CssClassPartials>,
    ) {
        if let Some(extra) = specified
            .and_then(|partials| partials.get(&ThemeAttr::Extra))
            .or_else(|| defaults.and_then(|partials| partials.get(&ThemeAttr::Extra)))
        {
            extra.split_whitespace().for_each(|extra_class| {
                css_classes_builder.append(extra_class);
            });
        }
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
    defaults: Option<&'attr CssClassPartials>,
    specified: Option<&'attr CssClassPartials>,
) -> Option<(ThemeAttr, &'attr str)> {
    attr_keys.iter().find_map(|attr_key| {
        specified
            .and_then(|partials| partials.get(attr_key))
            .or_else(|| defaults.and_then(|partials| partials.get(attr_key)))
            .map(|value| (*attr_key, value.as_str()))
    })
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
    stroke_width_keys: &'static [ThemeAttr],
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
            stroke_width_keys: &[ThemeAttr::StrokeWidth],
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
            stroke_width_keys: &[ThemeAttr::StrokeWidth],
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
            stroke_width_keys: &[ThemeAttr::StrokeWidth],
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
            stroke_width_keys: &[ThemeAttr::StrokeWidth],
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
            stroke_width_keys: &[ThemeAttr::StrokeWidth],
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
            stroke_width_keys: &[ThemeAttr::OutlineWidth],
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
            stroke_width_keys: &[ThemeAttr::OutlineWidth],
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
            stroke_width_keys: &[ThemeAttr::OutlineWidth],
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
            stroke_width_keys: &[ThemeAttr::OutlineWidth],
            fn_css_classes,
        }
    }
}

/// Stroke attributes that were specified but not used, due to not a complete
/// set of attributes provided to compute a CSS class.
///
/// The `ThemeAttr` in each `NoChange` variant is the `ThemeAttr` that is
/// specified, i.e. one of the fallbacks.
enum StrokeClassAppendResult<'value> {
    Added,
    NoChange {
        style: Option<(ThemeAttr, &'value str)>,
        width: Option<(ThemeAttr, &'value str)>,
        color: Option<(ThemeAttr, &'value str)>,
        shade: Option<(ThemeAttr, &'value str)>,
    },
    NoAttrsSpecified,
}

/// Fill attributes that were specified but not used, due to not a complete set
/// of attributes provided to compute a CSS class.
///
/// The `ThemeAttr` in each `NoChange` variant is the `ThemeAttr` that is
/// specified, i.e. one of the fallbacks.
enum FillClassAppendResult<'value> {
    Added,
    NoChange {
        color: Option<(ThemeAttr, &'value str)>,
        shade: Option<(ThemeAttr, &'value str)>,
    },
    NoAttrsSpecified,
}
