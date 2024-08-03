use crate::common::TagId;

/// The purpose of CSS classes' styles -- element regular styles, or when a tag
/// is interacted with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleFor<'tag> {
    /// The element in its regular state.
    Regular,
    /// The element when a tag it is part of is focused.
    TagFocus(&'tag TagId),
}
