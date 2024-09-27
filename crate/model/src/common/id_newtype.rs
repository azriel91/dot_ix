/// Implements common behaviour for an ID type.
///
/// The implemented behaviour includes:
///
/// * `IdType::new`
/// * `IdType::new_unchecked`
/// * `IdType::is_valid_id`
/// * `IdType::into_inner`
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
/// pub struct MyIdType(Cow<'static, str>);
///
/// crate::id_newtype!(
///     MyIdType,           // Name of the ID type
///     MyIdTypeInvalidFmt, // Name of the invalid value error
///     my_id_type,         // Name of the static check macro
/// );
/// ```
macro_rules! id_newtype {
    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the [`", stringify!($macro_name), "!`] macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            #[doc = concat!("[`", stringify!($macro_name), "!`]: dot_ix_static_check_macros::profile")]
            pub fn new(s: &'static str) -> Result<Self, $ty_err_name<'static>> {
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

            /// Returns whether the provided `&str` is a valid station identifier.
            pub fn is_valid_id(proposed_id: &str) -> bool {
                let mut chars = proposed_id.chars();
                let first_char = chars.next();
                let first_char_valid = first_char
                    .map(|c| c.is_ascii_alphabetic() || c == '_')
                    .unwrap_or(false);
                let remainder_chars_valid =
                    chars.all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit());

                first_char_valid && remainder_chars_valid
            }

            /// Returns the inner `Cow<'static, str>`.
            pub fn into_inner(self) -> Cow<'static, str> {
                self.0
            }

            /// Returns the `&str` held by this ID.
            pub fn as_str(&self) -> &str {
                &self.0
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
                if Self::is_valid_id(&s) {
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
                if Self::is_valid_id(s) {
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
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Owned(String::from(s))))
                } else {
                    let s = std::borrow::Cow::Owned(String::from(s));
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl std::borrow::Borrow<str> for $ty_name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl<'s> std::borrow::Borrow<str> for &'s $ty_name {
            fn borrow(&self) -> &str {
                &self.0
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

pub(crate) use id_newtype;
