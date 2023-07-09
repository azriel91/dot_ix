use serde::{Deserialize, Serialize};

/// Basic node info.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tag {
    /// Shorthand, only tag name is provided.
    Name(
        /// One line, plain text display.
        String,
    ),
    /// Fields all provided.
    Info {
        /// One line, plain text display.
        name: String,
        /// Plain text description, used as tooltip.
        #[serde(default)]
        desc: Option<String>,
    },
}
