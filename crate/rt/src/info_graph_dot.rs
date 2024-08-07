use std::collections::HashMap;

use dot_ix_model::{
    common::{AnyId, EdgeId, NodeHierarchy, NodeId},
    info_graph::GraphStyle,
    theme::{ColorParams, CssClassesBuilder, HighlightState, LineParams, Themeable},
};

#[derive(Clone)]
pub struct InfoGraphDot<'graph> {
    pub graph_style: GraphStyle,
    pub node_id_to_hierarchy: &'graph HashMap<&'graph NodeId, &'graph NodeHierarchy>,
    pub node_ids: Vec<&'graph NodeId>,
    pub edge_ids: Vec<&'graph EdgeId>,
}

impl<'graph> InfoGraphDot<'graph> {
    fn el_prefix(&self, node_id: &AnyId) -> &str {
        let is_cluster = self
            .node_id_to_hierarchy
            .get(node_id.as_str())
            .map(|hierarchy| !hierarchy.is_empty())
            .unwrap_or(false);

        if is_cluster {
            return "[&>path]:";
        }

        match self.graph_style {
            GraphStyle::Box => "[&>path]:",
            GraphStyle::Circle => "[&>ellipse]:",
        }
    }
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
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        path_color_classes(builder, color_params, "outline", self.el_prefix(node_id));
        outline_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            self.el_prefix(node_id),
        );
    }

    fn node_stroke_classes(
        &self,
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        line_params: LineParams<'_>,
    ) {
        let LineParams {
            color_params,
            line_width,
            line_style,
        } = line_params;

        path_color_classes(builder, color_params, "stroke", self.el_prefix(node_id));
        border_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            self.el_prefix(node_id),
        );
    }

    fn node_fill_classes(
        &self,
        node_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    ) {
        path_color_classes(builder, color_params, "fill", self.el_prefix(node_id));
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

        el_color_classes(builder, color_params, "outline", "");
        outline_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "",
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

        path_color_classes(builder, color_params, "stroke", "[&>path]:");
        polygon_color_classes(builder, color_params, "stroke");
        border_style_classes(
            builder,
            color_params.highlight_state,
            line_width,
            line_style,
            "[&>path]:",
        );
    }

    fn edge_fill_classes(
        &self,
        _edge_id: &AnyId,
        builder: &mut CssClassesBuilder,
        color_params: ColorParams<'_>,
    ) {
        // deliberately don't have `"fill"` for `path`, because that adds a thin line
        // when the path is styled as dashed
        polygon_color_classes(builder, color_params, "fill");
    }
}

fn outline_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    line_width: &str,
    line_style: &str,
    el_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    builder
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{line_width}"
        ))
        .append(&format!(
            "{el_prefix}{highlight_prefix}outline-{line_style}"
        ));
}

/// Appends SVG stroke classes that emulate HTML border styles.
fn border_style_classes(
    builder: &mut CssClassesBuilder,
    highlight_state: HighlightState,
    line_width: &str,
    line_style: &str,
    el_prefix: &str,
) {
    let highlight_prefix = highlight_prefix(highlight_state);

    match line_style {
        "none" => {}
        "solid" => {
            builder.append(&format!("{el_prefix}{highlight_prefix}stroke-{line_width}"));
        }
        "dashed" => {
            builder
                .append(&format!("{el_prefix}{highlight_prefix}stroke-{line_width}"))
                .append(&format!(
                    "{el_prefix}{highlight_prefix}[stroke-dasharray:3]"
                ));
        }
        "dotted" => {
            builder
                .append(&format!("{el_prefix}{highlight_prefix}stroke-{line_width}"))
                .append(&format!(
                    "{el_prefix}{highlight_prefix}[stroke-dasharray:2]"
                ));
        }
        line_style if line_style.starts_with("dasharray:") => {
            builder
                .append(&format!("{el_prefix}{highlight_prefix}stroke-{line_width}"))
                .append(&format!(
                    "{el_prefix}{highlight_prefix}[stroke-{line_style}]"
                ));
        }
        _ => {
            // TODO #29: warn user of unknown value.
        }
    };
}

fn path_color_classes(
    builder: &mut CssClassesBuilder,
    color_params: ColorParams<'_>,
    stroke_or_fill: &str,
    el_prefix: &str,
) {
    el_color_classes(builder, color_params, stroke_or_fill, el_prefix);
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
