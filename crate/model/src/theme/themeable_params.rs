use crate::theme::HighlightState;

/// Parameters to compute `CssClasses`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeableParams<'params> {
    /// Whether the element is in the normal, focused, hovered, or active state.
    pub highlight_state: HighlightState,
    /// Name of the colour palette to apply, e.g. `"slate"`.
    pub color: &'params str,
    /// Shade number to apply, e.g. `"600"`.
    pub shade: &'params str,
}

impl<'params> ThemeableParams<'params> {
    /// Returns a new `ThemeableParams`.
    pub fn new(highlight_state: HighlightState, color: &'params str, shade: &'params str) -> Self {
        Self {
            highlight_state,
            color,
            shade,
        }
    }
}
