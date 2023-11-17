use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for a `TailwindClass`, `Cow<'static, str>` newtype.
///
/// Must never contain whitespace, and should be a [tailwind] class.
///
/// [tailwind]: https://tailwindcss.com/
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TailwindClass(Cow<'static, str>);

crate::model::common::string_no_whitespace_newtype!(
    TailwindClass,
    TailwindClassInvalidFmt,
    tailwind_class
);
