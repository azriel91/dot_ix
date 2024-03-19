use serde::{Deserialize, Serialize};

/// Whether an element is focused, hovered, or active.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HighlightState {
    /// The element is not focused, and the cursor is not positioned over it.
    Normal,
    /// The element is focused, and the cursor is not positioned over it.
    Focus,
    /// The cursor is positioned over the element.
    Hover,
    /// The element is clicked / pressed.
    Active,
}
