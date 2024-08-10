use serde::{Deserialize, Serialize};

/// Defines the path, width, and height of an image for GraphViz.
///
/// Without this, the `<image>` element is not rendered for a node.
///
/// See [`Image`].
///
/// [`Image`]: https://hpcc-systems.github.io/hpcc-js-wasm/graphviz/interfaces/Image.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GraphvizImage {
    /// URL to the image. This may be a data URL.
    ///
    /// # Examples
    ///
    /// Hyperlinked image:
    ///
    /// ```text
    /// https://example.com/image.png
    /// ```
    ///
    /// Inline image:
    ///
    /// ```text
    /// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABkAAAAZBAMAAAA2x5hQAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAADUExURUeK/z7BOdMAAAAJcEhZcwAADsIAAA7CARUoSoAAAAAOSURBVCjPYxgFNAMMDAABXgABAvs87wAAAABJRU5ErkJggg==
    /// ```
    pub path: String,
    /// Width that GraphViz scales the image, e.g. `"50px"`.
    pub width: String,
    /// Height that GraphViz scales the image, e.g. `"50px"`.
    pub height: String,
}

impl GraphvizImage {
    /// Returns a new `GraphvizImage`.
    pub fn new(path: String, width: String, height: String) -> Self {
        Self {
            path,
            width,
            height,
        }
    }

    /// Returns the URL to the image. This may be a data URL.
    ///
    /// # Examples
    ///
    /// Hyperlinked image:
    ///
    /// ```text
    /// https://example.com/image.png
    /// ```
    ///
    /// Inline image:
    ///
    /// ```text
    /// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABkAAAAZBAMAAAA2x5hQAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAADUExURUeK/z7BOdMAAAAJcEhZcwAADsIAAA7CARUoSoAAAAAOSURBVCjPYxgFNAMMDAABXgABAvs87wAAAABJRU5ErkJggg==
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the width that GraphViz scales the image, e.g. `"50px"`.
    pub fn width(&self) -> &str {
        &self.width
    }

    /// Returns the height that GraphViz scales the image, e.g. `"50px"`.
    pub fn height(&self) -> &str {
        &self.height
    }
}
