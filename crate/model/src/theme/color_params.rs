use crate::theme::HighlightState;

/// Parameters to compute colour related `CssClasses`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorParams<'params> {
    /// Whether the element is in the normal, focused, hovered, or active state.
    pub highlight_state: HighlightState,
    /// Name of the colour palette to apply, e.g. `"slate"`.
    pub color: &'params str,
    /// Shade number to apply, e.g. `"600"`.
    pub shade: &'params str,
}

impl<'params> ColorParams<'params> {
    /// Returns a new `FillParams`.
    pub fn new(highlight_state: HighlightState, color: &'params str, shade: &'params str) -> Self {
        Self {
            highlight_state,
            color,
            shade,
        }
    }
}
