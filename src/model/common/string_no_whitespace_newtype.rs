/// Implements common behaviour for a String newtype that cannot contain
/// whitespace.
///
/// The implemented behaviour includes:
///
/// * `StringType::new`
/// * `StringType::new_unchecked`
/// * `StringType::is_valid_value`
/// * `StringType::into_inner`
/// * `std::ops::Deref`
/// * `std::ops::DerefMut`
/// * `std::fmt::Display`
/// * `std::str::FromStr`
/// * `TryFrom<String>`
/// * `TryFrom<&'static str>`
///
/// A separate error type is also generated, which indicates an invalid value
/// when the ID type is instantiated with `new`.
///
/// # Usage
///
/// ```rust
/// use std::borrow::Cow;
///
/// // replace this with your ID type's macro
/// use dot_ix_static_check_macros::my_id_type;
/// use serde::{Deserialize, Serialize};
///
/// // Rename your ID type
/// #[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
/// pub struct MyStringType(Cow<'static, str>);
///
/// crate::id_newtype!(
///     MyStringType,           // Name of the ID type
///     MyStringTypeInvalidFmt, // Name of the invalid value error
///     my_id_type,             // Name of the static check macro
/// );
/// ```
macro_rules! string_no_whitespace_newtype {
    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the [`", stringify!($macro_name), "!`] macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            #[doc = concat!("[`", stringify!($macro_name), "!`]: dot_ix_static_check_macros::profile")]
            pub fn new(s: &'static str) -> Result<Self, $ty_err_name> {
                Self::try_from(s)
            }

            #[doc = concat!("Returns a new `", stringify!($ty_name), "`.")]
            ///
            #[doc = concat!("Most users should use the [`", stringify!($macro_name), "!`] macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            #[doc = concat!("[`", stringify!($macro_name), "!`]: dot_ix_static_check_macros::profile")]
            #[doc(hidden)]
            pub const fn new_unchecked(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
            }

            /// Returns whether the provided `&str` is a string without whitespace.
            pub fn is_valid_value(proposed_id: &str) -> bool {
                !proposed_id.chars().any(char::is_whitespace)
            }

            /// Returns the inner `Cow<'static, str>`.
            pub fn into_inner(self) -> Cow<'static, str> {
                self.0
            }
        }

        impl std::ops::Deref for $ty_name {
            type Target = std::borrow::Cow<'static, str>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $ty_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<String> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: String) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_value(&s) {
                    Ok($ty_name(std::borrow::Cow::Owned(s)))
                } else {
                    let s = std::borrow::Cow::Owned(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl TryFrom<&'static str> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: &'static str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_value(s) {
                    Ok($ty_name(std::borrow::Cow::Borrowed(s)))
                } else {
                    let s = std::borrow::Cow::Borrowed(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl std::str::FromStr for $ty_name {
            type Err = $ty_err_name<'static>;

            fn from_str(s: &str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_value(s) {
                    Ok($ty_name(std::borrow::Cow::Owned(String::from(s))))
                } else {
                    let s = std::borrow::Cow::Owned(String::from(s));
                    Err($ty_err_name::new(s))
                }
            }
        }

        #[doc = concat!("Error indicating `", stringify!($ty_name), "` provided is not in the correct format.")]
        #[derive(Debug, PartialEq, Eq)]
        pub struct $ty_err_name<'s> {
            /// String that was provided for the `$ty_name`.
            value: std::borrow::Cow<'s, str>,
        }

        impl<'s> $ty_err_name<'s> {
            #[doc = concat!("Returns a new `", stringify!($ty_err_name), "` error.")]
            pub fn new(value: std::borrow::Cow<'s, str>) -> Self {
                Self { value }
            }

            #[doc = concat!("Returns the value that failed to be parsed as a [`", stringify!($ty_name), "`].")]
            pub fn value(&self) -> &std::borrow::Cow<'s, str> {
                &self.value
            }
        }

        impl<'s> std::fmt::Display for $ty_err_name<'s> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "`{value}` is not a valid `{ty_name}`.\n\
                    `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
                    ty_name = stringify!($ty_name),
                    value = self.value
                )
            }
        }

        impl<'s> std::error::Error for $ty_err_name<'s> {}
    };
}

pub(crate) use string_no_whitespace_newtype;
