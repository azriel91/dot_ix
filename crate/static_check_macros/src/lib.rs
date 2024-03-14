use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident};

use self::lit_str_maybe::LitStrMaybe;

mod lit_str_maybe;

/// Returns a `const NodeId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `NodeId` at compile time:
///
/// ```rust
/// # use dot_ix_static_check_macros::node_id;
/// #
/// let _my_node_id: dot_ix::model::common::NodeId = node_id!("valid_id"); // Ok!
/// //
/// #
/// # pub mod dot_ix {
/// #     pub mod model {
/// #         pub mod common {
/// #             pub struct NodeId(&'static str);
/// #             impl NodeId {
/// #                 pub fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// #             }
/// #         }
/// #     }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use dot_ix_static_check_macros::node_id;
///
/// let _my_node_id: dot_ix::model::common::NodeId = node_id!("-invalid_id"); // Compile error
/// //                                               ^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `NodeId`.
/// //        `NodeId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # pub mod dot_ix {
/// #     pub mod model {
/// #         pub mod common {
/// #             pub struct NodeId(&'static str);
/// #             impl NodeId {
/// #                 pub fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// #             }
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn node_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ensure_valid_id(&parse_macro_input!(input as LitStrMaybe), "NodeId", None).into()
}

/// Returns a `const EdgeId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `EdgeId` at compile time:
///
/// ```rust,ignore
/// # use dot_ix_static_check_macros::edge_id;
/// #
/// let _my_flow: dot_ix::model::common::EdgeId = edge_id!("valid_id"); // Ok!
/// //
/// #
/// # pub mod dot_ix {
/// #     pub mod model {
/// #         pub mod common {
/// #             pub struct EdgeId(&'static str);
/// #             impl EdgeId {
/// #                 pub fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// #             }
/// #         }
/// #     }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,ignore
/// # use dot_ix_static_check_macros::edge_id;
///
/// let _my_flow: dot_ix::model::common::EdgeId = edge_id!("-invalid_id"); // Compile error
/// //                                            ^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `EdgeId`.
/// //        `EdgeId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # pub mod dot_ix {
/// #     pub mod model {
/// #         pub mod common {
/// #             pub struct EdgeId(&'static str);
/// #             impl EdgeId {
/// #                 pub fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// #             }
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn edge_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ensure_valid_id(&parse_macro_input!(input as LitStrMaybe), "EdgeId", None).into()
}

fn ensure_valid_id(
    proposed_id: &LitStrMaybe,
    ty_name: &str,
    default: Option<&str>,
) -> proc_macro2::TokenStream {
    let proposed_id = proposed_id.as_ref().map(|lit_str| lit_str.value());
    let proposed_id = proposed_id.as_deref().or(default);

    if let Some(proposed_id) = proposed_id {
        if is_valid_id(proposed_id) {
            let ty_name = Ident::new(ty_name, Span::call_site());
            quote!( dot_ix::model::common:: #ty_name ::new_unchecked( #proposed_id ))
        } else {
            let message = format!(
                "\"{proposed_id}\" is not a valid `{ty_name}`.\n\
                `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
            );
            compile_fail(message)
        }
    } else {
        let message = format!(
            "`` is not a valid `{ty_name}`.\n\
            `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
        );
        compile_fail(message)
    }
}

fn compile_fail(message: String) -> proc_macro2::TokenStream {
    quote!(compile_error!(#message))
}

fn is_valid_id(proposed_id: &str) -> bool {
    let mut chars = proposed_id.chars();
    let first_char = chars.next();
    let first_char_valid = first_char
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false);
    let remainder_chars_valid =
        chars.all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit());

    first_char_valid && remainder_chars_valid
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::LitStr;

    use crate::LitStrMaybe;

    use super::ensure_valid_id;

    #[test]
    fn name_beginning_with_underscore_is_valid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("_", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            r#"dot_ix :: model :: common :: Ty :: new_unchecked ("_")"#,
            tokens.to_string()
        );
    }

    #[test]
    fn name_beginning_with_alpha_is_valid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("a", Span::call_site()))),
            "Ty",
            None,
        );
        assert_eq!(
            r#"dot_ix :: model :: common :: Ty :: new_unchecked ("a")"#,
            tokens.to_string()
        );

        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("A", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            r#"dot_ix :: model :: common :: Ty :: new_unchecked ("A")"#,
            tokens.to_string()
        );
    }

    #[test]
    fn name_beginning_with_number_is_invalid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("1", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            "compile_error ! (\"\\\"1\\\" is not a valid `Ty`.\\n\
            `Ty`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.\")",
            tokens.to_string()
        );
    }

    #[test]
    fn name_containing_space_is_invalid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("a b", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            "compile_error ! (\"\\\"a b\\\" is not a valid `Ty`.\\n\
            `Ty`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.\")",
            tokens.to_string()
        );
    }

    #[test]
    fn name_containing_hyphen_is_invalid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("a-b", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            "compile_error ! (\"\\\"a-b\\\" is not a valid `Ty`.\\n\
            `Ty`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.\")",
            tokens.to_string()
        );
    }

    #[test]
    fn name_empty_string_is_invalid() {
        let tokens = ensure_valid_id(
            &LitStrMaybe(Some(LitStr::new("", Span::call_site()))),
            "Ty",
            None,
        );

        assert_eq!(
            "compile_error ! (\"\\\"\\\" is not a valid `Ty`.\\n\
            `Ty`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.\")",
            tokens.to_string()
        );
    }

    #[test]
    fn name_none_is_invalid() {
        let tokens = ensure_valid_id(&LitStrMaybe(None), "Ty", None);

        assert_eq!(
            "compile_error ! (\"`` is not a valid `Ty`.\\n\
            `Ty`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.\")",
            tokens.to_string()
        );
    }
}
