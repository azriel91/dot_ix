use leptos_router::AsPath;

/// String newtype to allow a formatted string to be used as a leptos `Route`
/// path.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringSegment(pub String);

impl StringSegment {
    /// Returns a new `StringSegment`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl AsPath for StringSegment {
    fn as_path(&self) -> &'static str {}
}
