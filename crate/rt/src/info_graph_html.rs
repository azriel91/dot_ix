use std::borrow::Cow;

use dot_ix_model::{
    common::{AnyId, EdgeId, NodeId},
    theme::{ColorParams, CssClassesBuilder, HighlightState, LineParams, Themeable},
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
        _node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        el_color_classes(builder, color_params, "outline");
        line_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "outline",
        );
    }

    fn node_stroke_classes(
        &self,
        _node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        el_color_classes(builder, color_params, "border");
        line_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "border",
        );
    }

    fn node_fill_classes(
        &self,
        _node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    ) {
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
        _edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        el_color_classes(builder, color_params, "outline");
        line_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "outline",
        );
    }

    fn edge_stroke_classes(
        &self,
        _edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        el_color_classes(builder, color_params, "border");
        line_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "border",
        );
    }

    fn edge_fill_classes(
        &self,
        _edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    ) {
        el_color_classes(builder, color_params, "bg");
    }
}

fn line_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    line_width: &str,
    line_style: &str,
    stroke_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);
    let line_width = if line_style == "dotted" {
        line_width_increment(line_width)
    } else {
        Cow::Borrowed(line_width)
    };

    builder
        .append(&format!(
            "{highlight_prefix}{stroke_prefix}-[{line_width}px]"
        ))
        .append(&format!("{highlight_prefix}{stroke_prefix}-{line_style}"));
}

/// Increments 1 to the stroke width if parseable.
///
/// This exists because SVG and HTML's rendering of stroke-1 vs border-1
/// differ:
///
/// * SVG's stroke-1 looks similar to HTML's border-2
fn line_width_increment(line_width: &str) -> Cow<'_, str> {
    let (Ok(line_width) | Err(line_width)) = line_width
        .parse::<u32>()
        .map(|w| Cow::Owned(w.saturating_add(1).to_string()))
        .map_err(|_| Cow::Borrowed(line_width));
    line_width
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
