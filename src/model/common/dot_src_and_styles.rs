use serde::{Deserialize, Serialize};

/// Graphviz dot source and CSS styles.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DotSrcAndStyles {
    /// Graphviz dot source to run in `dot`.
    pub dot_src: String,
    /// Tailwind CSS styles to put into `<styles>..</styles>`.
    pub styles: String,
}
