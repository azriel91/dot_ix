use serde::{Deserialize, Serialize};

/// The style of graph to render.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphStyle {
    /// A rectangle is rendered for each node, with labels within them.
    #[default]
    Boxes,
    /// A circle is rendered for each node, with labels next to them.
    Circle,
}
