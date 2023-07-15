use serde::{Deserialize, Serialize};

/// Name and description for a node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    /// One line, plain text display.
    name: String,
    /// Plain text description.
    #[serde(default)]
    desc: Option<String>,
}

impl NodeInfo {
    /// Returns this node's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the description.
    pub fn desc(&self) -> Option<&str> {
        self.desc.as_deref()
    }
}
