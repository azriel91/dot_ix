use crate::theme::ColorParams;

/// Parameters to compute `CssClasses`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrokeParams<'params> {
    /// Parameters to compute colour related `CssClasses`.
    pub color_params: ColorParams<'params>,
    /// Width that the line should be, e.g. `"1"`.
    pub stroke_width: &'params str,
    /// Style that the line should be, e.g. `"dashed"`.
    pub stroke_style: &'params str,
}

impl<'params> StrokeParams<'params> {
    /// Returns a new `StrokeParams`.
    pub fn new(
        color_params: ColorParams<'params>,
        stroke_width: &'params str,
        stroke_style: &'params str,
    ) -> Self {
        Self {
            color_params,
            stroke_width,
            stroke_style,
        }
    }
}
