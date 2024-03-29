use std::borrow::Cow;

use dot_ix_model::{
    common::{EdgeId, NodeId},
    theme::{ColorParams, CssClassesBuilder, HighlightState, StrokeParams, Themeable},
};

#[derive(Clone)]
pub struct InfoGraphHtml<'graph> {
    pub node_ids: Vec<&'graph NodeId>,
    pub edge_ids: Vec<&'graph EdgeId>,
}

impl<'graph> Themeable for InfoGraphHtml<'graph> {
    fn node_ids(&self) -> impl Iterator<Item = &NodeId>
    where
        Self: Sized,
    {
        self.node_ids.iter().copied()
    }

    fn node_outline_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        el_color_classes(builder, color_params, "outline");
        stroke_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "outline",
        );
    }

    fn node_stroke_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        el_color_classes(builder, color_params, "border");
        stroke_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "border",
        );
    }

    fn node_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        el_color_classes(builder, color_params, "bg");
    }

    fn edge_ids(&self) -> impl Iterator<Item = &EdgeId>
    where
        Self: Sized,
    {
        self.edge_ids.iter().copied()
    }

    fn edge_outline_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        el_color_classes(builder, color_params, "outline");
        stroke_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "outline",
        );
    }

    fn edge_stroke_classes(
        &self,
        builder: &mut CssClassesBuilder,
        stroke_params: StrokeParams<'_>,
    ) {
        let StrokeParams {
            color_params,
            stroke_width,
            stroke_style,
        } = stroke_params;

        el_color_classes(builder, color_params, "border");
        stroke_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "border",
        );
    }

    fn edge_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        el_color_classes(builder, color_params, "bg");
    }
}

fn stroke_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    stroke_width: &str,
    stroke_style: &str,
    stroke_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);
    let stroke_width = if stroke_style == "dotted" {
        stroke_width_increment(stroke_width)
    } else {
        Cow::Borrowed(stroke_width)
    };

    builder
        .append(&format!(
            "{highlight_prefix}{stroke_prefix}-[{stroke_width}px]"
        ))
        .append(&format!("{highlight_prefix}{stroke_prefix}-{stroke_style}"));
}

/// Increments 1 to the stroke width if parseable.
///
/// This exists becauase SVG and HTML's rendering of stroke-1 vs border-1
/// differ:
///
/// * SVG's stroke-1 looks similar to HTML's border-2
fn stroke_width_increment(stroke_width: &str) -> Cow<'_, str> {
    let (Ok(stroke_width) | Err(stroke_width)) = stroke_width
        .parse::<u32>()
        .map(|w| Cow::Owned(w.saturating_add(1).to_string()))
        .map_err(|_| Cow::Borrowed(stroke_width));
    stroke_width
}

fn el_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
) {
    let ColorParams {
        highlight_state,
        color,
        shade,
    } = color_params;

    let highlight_prefix = highlight_prefix(highlight_state);
    builder.append(&format!(
        "{highlight_prefix}{stroke_or_fill}-{color}-{shade}"
    ));
}

fn highlight_prefix(highlight_state: HighlightState) -> &'static str {
    match highlight_state {
        HighlightState::Normal => "",
        HighlightState::Focus => "focus:",
        HighlightState::FocusHover => "focus:hover:",
        HighlightState::Hover => "hover:",
        HighlightState::Active => "focus:active:",
    }
}
