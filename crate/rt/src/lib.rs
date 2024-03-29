//! Runtime logic for the `dot_ix` diagramming application.

pub use crate::{info_graph_dot::InfoGraphDot, into_graphviz_dot_src::IntoGraphvizDotSrc};

mod info_graph_dot;
mod into_graphviz_dot_src;

cfg_if::cfg_if! { if #[cfg(feature = "info_graph_html")] {
    pub use crate::info_graph_html::InfoGraphHtml;

    mod info_graph_html;
}}
