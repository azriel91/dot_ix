#![allow(non_snake_case)] // Components are all PascalCase.

//! Web components provided by the `dot_ix` library.

pub use crate::{
    dot_svg::DotSvg,
    error_template::{AppError, ErrorTemplate},
};

mod dot_svg;
mod error_template;

#[cfg(feature = "flex_diag")]
pub use crate::flex_diag::FlexDiag;

#[cfg(feature = "flex_diag")]
mod flex_diag;
