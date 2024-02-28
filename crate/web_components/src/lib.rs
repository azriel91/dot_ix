#![allow(non_snake_case)] // Components are all PascalCase.

//! Web components provided by the `dot_ix` library.

pub use crate::{
    dot_svg::DotSvg,
    error_template::{AppError, ErrorTemplate},
};

mod dot_svg;
mod error_template;

#[cfg(feature = "div_diag")]
pub use crate::div_diag::DivDiag;

#[cfg(feature = "div_diag")]
mod div_diag;
