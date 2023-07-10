use serde::{Deserialize, Serialize};

/// Basic node info.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeInfo {
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

impl NodeInfo {
    pub fn name(&self) -> &str {
        match self {
            NodeInfo::Name(name) => name.as_str(),
            NodeInfo::Info { name, desc: _ } => name.as_str(),
        }
    }
}
