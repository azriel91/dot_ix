use serde::{Deserialize, Serialize};

/// Basic node info.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    /// Shorthand, only name is provided.
    Name(
        /// One line, plain text display.
        String,
    ),
    /// Fields all provided.
    Info {
        /// One line, plain text display.
        name: String,
        /// Plain text description.
        #[serde(default)]
        desc: Option<String>,
    },
}
