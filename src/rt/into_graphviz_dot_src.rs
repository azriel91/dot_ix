use crate::model::common::GraphvizDotTheme;

mod info_graph;

/// Generates GraphViz Dot source that can be rendered using `dot`.
pub trait IntoGraphvizDotSrc {
    fn into(self, theme: &GraphvizDotTheme) -> String;
}
