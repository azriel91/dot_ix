use serde::{Deserialize, Serialize};

/// Name and description for a tag.
///
/// Tags are used to indicate topics, like hashtags on social media platforms.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    /// One line, plain text display.
    name: String,
    /// Plain text description, used as tooltip.
    #[serde(default)]
    desc: Option<String>,
}

impl Tag {
    /// Returns this tag's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the description.
    pub fn desc(&self) -> Option<&str> {
        self.desc.as_deref()
    }
}
