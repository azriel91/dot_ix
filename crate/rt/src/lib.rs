//! Runtime logic for the `dot_ix` diagramming application.

pub use self::{info_graph_dot::InfoGraphDot, into_graphviz_dot_src::IntoGraphvizDotSrc};

mod info_graph_dot;
mod into_graphviz_dot_src;
