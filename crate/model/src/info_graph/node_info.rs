use serde::{Deserialize, Serialize};

/// Name and description for a node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Emoji to display.
    #[serde(default)]
    emoji: Option<String>,
    /// One line, plain text display.
    name: String,
    /// Plain text description.
    #[serde(default)]
    desc: Option<String>,
}

impl NodeInfo {
    /// Returns a new `NodeInfo`.
    pub fn new(name: String) -> Self {
        Self {
            emoji: None,
            name,
            desc: None,
        }
    }

    /// Sets the emoji to display.
    pub fn with_emoji(mut self, emoji: Option<String>) -> Self {
        self.emoji = emoji;
        self
    }

    /// Sets the plain text description.
    pub fn with_desc(mut self, desc: Option<String>) -> Self {
        self.desc = desc;
        self
    }

    /// Returns the emoji to display.
    pub fn emoji(&self) -> Option<&str> {
        self.emoji.as_deref()
    }

    /// Returns this node's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the description.
    pub fn desc(&self) -> Option<&str> {
        self.desc.as_deref()
    }
}
