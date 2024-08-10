use serde::{Deserialize, Serialize};

use crate::common::dot_src_and_styles::GraphvizImage;

/// Options to pass to graphviz when rendering.
///
/// Currently this is used to pass inline images.
///
/// See [`Options`].
///
/// [`Options`]: https://hpcc-systems.github.io/hpcc-js-wasm/graphviz/interfaces/Options.html
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct GraphvizOpts {
    /// The list of [`images`] to pass to `graphviz.layout`.
    ///
    /// [`images`]: https://hpcc-systems.github.io/hpcc-js-wasm/graphviz/interfaces/Image.html
    pub images: Vec<GraphvizImage>,
}

impl GraphvizOpts {
    /// Returns a new `GraphvizOpts` object.
    pub fn new(images: Vec<GraphvizImage>) -> Self {
        Self { images }
    }

    /// Returns the list of [`images`] to pass to `graphviz.layout`.
    ///
    /// [`images`]: https://hpcc-systems.github.io/hpcc-js-wasm/graphviz/interfaces/Image.html
    pub fn images(&self) -> &[GraphvizImage] {
        &self.images
    }
}
