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

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Name(name) => name.as_str(),
            Node::Info { name, desc: _ } => name.as_str(),
        }
    }
}
