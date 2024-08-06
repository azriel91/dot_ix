use crate::theme::ColorParams;

/// Parameters to compute `CssClasses`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineParams<'params> {
    /// Parameters to compute colour related `CssClasses`.
    pub color_params: ColorParams<'params>,
    /// Width that the line should be, e.g. `"1"`.
    ///
    /// Corresponds to either `stroke-width` or `outline-width`.
    pub line_width: &'params str,
    /// Style that the line should be, e.g. `"dashed"`.
    pub line_style: &'params str,
}

impl<'params> LineParams<'params> {
    /// Returns a new `LineParams`.
    pub fn new(
        color_params: ColorParams<'params>,
        line_width: &'params str,
        line_style: &'params str,
    ) -> Self {
        Self {
            color_params,
            line_width,
            line_style,
        }
    }
}
