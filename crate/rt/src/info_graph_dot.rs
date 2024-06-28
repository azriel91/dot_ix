use dot_ix_model::{
    common::{EdgeId, NodeId},
    theme::{ColorParams, CssClassesBuilder, HighlightState, StrokeParams, Themeable},
};

#[derive(Clone)]
pub struct InfoGraphDot<'graph> {
    pub node_ids: Vec<&'graph NodeId>,
    pub edge_ids: Vec<&'graph EdgeId>,
}

impl<'graph> Themeable for InfoGraphDot<'graph> {
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

        path_color_classes(builder, color_params, "outline");
        outline_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "[&>path]:",
        );
        outline_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "[&>ellipse]:",
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

        path_color_classes(builder, color_params, "stroke");
        border_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
        );
    }

    fn node_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        path_color_classes(builder, color_params, "fill");
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

        el_color_classes(builder, color_params, "outline", "");
        outline_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
            "",
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

        path_color_classes(builder, color_params, "stroke");
        polygon_color_classes(builder, color_params, "stroke");
        border_style_classes(
            builder,
            color_params.highlight_state,
            stroke_width,
            stroke_style,
        );
    }

    fn edge_fill_classes(&self, builder: &mut CssClassesBuilder, color_params: ColorParams<'_>) {
        // deliberately don't have `"fill"` for `path`, because that adds a thin line
        // when the path is styled as dashed
        polygon_color_classes(builder, color_params, "fill");
    }
}

fn outline_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    stroke_width: &str,
    stroke_style: &str,
    el_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    builder
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{stroke_width}"
        ))
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{stroke_style}"
        ));
}

/// Appends SVG stroke classes that emulate HTML border styles.
fn border_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    stroke_width: &str,
    stroke_style: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    match stroke_style {
        "none" => {}
        "solid" => {
            builder.append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"));
        }
        "dashed" => {
            builder
                .append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"))
                .append(&format!("[&>path]:{highlight_prefix}[stroke-dasharray:3]"));
        }
        "dotted" => {
            builder
                .append(&format!("[&>path]:{highlight_prefix}stroke-{stroke_width}"))
                .append(&format!("[&>path]:{highlight_prefix}[stroke-dasharray:2]"));
        }
        _ => {}
    };
}

fn path_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
) {
    el_color_classes(builder, color_params, stroke_or_fill, "[&>path]:");
    el_color_classes(builder, color_params, stroke_or_fill, "[&>ellipse]:");
}

fn polygon_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
) {
    el_color_classes(builder, color_params, stroke_or_fill, "[&>polygon]:");
}

fn el_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
    el_prefix: &str,
) {
    let ColorParams {
        highlight_state,
        color,
        shade,
    } = color_params;

    let highlight_prefix = highlight_prefix(highlight_state);
    builder.append(&format!(
        "{el_prefix}{highlight_prefix}{stroke_or_fill}-{color}-{shade}"
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
