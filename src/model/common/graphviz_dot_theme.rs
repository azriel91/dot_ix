#[derive(Clone, Debug, PartialEq)]
pub struct GraphvizDotTheme {
    edge_color: &'static str,

    // Node attributes:
    //
    // <https://graphviz.org/docs/nodes/>
    node_text_color: &'static str,
    /// Width of a node, but it is allowed to expand.
    node_width: f64,
    /// Height of a node, but it is allowed to expand.
    node_height: f64,
    /// Left and right margin in inches.
    ///
    /// Default: `0.04`.
    ///
    /// Graphviz default: `0.11`.
    node_margin_x: f64,
    /// Top and bottom margin in inches.
    ///
    /// Default: `0.04`.
    ///
    /// Graphviz default: `0.055`.
    node_margin_y: f64,
    plain_text_color: &'static str,
    emoji_point_size: u32,
    node_point_size: u32,
    tag_width: f64,
    tag_height: f64,
    /// Left and right margin in inches.
    ///
    /// Default: `0.03`.
    ///
    /// Graphviz default: `0.11`.
    tag_margin_x: f64,
    /// Top and bottom margin in inches.
    ///
    /// Default: `0.02`.
    ///
    /// Graphviz default: `0.055`.
    tag_margin_y: f64,
    tag_point_size: u32,
    tag_classes: &'static str,
}

impl GraphvizDotTheme {
    pub fn new(
        edge_color: &'static str,
        node_text_color: &'static str,
        node_width: f64,
        node_height: f64,
        node_margin_x: f64,
        node_margin_y: f64,
        plain_text_color: &'static str,
        emoji_point_size: u32,
        node_point_size: u32,
        tag_width: f64,
        tag_height: f64,
        tag_margin_x: f64,
        tag_margin_y: f64,
        tag_point_size: u32,
        tag_classes: &'static str,
    ) -> Self {
        Self {
            edge_color,
            node_text_color,
            node_width,
            node_height,
            node_margin_x,
            node_margin_y,
            plain_text_color,
            emoji_point_size,
            node_point_size,
            tag_width,
            tag_height,
            tag_margin_x,
            tag_margin_y,
            tag_point_size,
            tag_classes,
        }
    }

    pub fn edge_color(&self) -> &str {
        self.edge_color
    }

    pub fn node_text_color(&self) -> &str {
        self.node_text_color
    }

    pub fn node_width(&self) -> f64 {
        self.node_width
    }

    pub fn node_height(&self) -> f64 {
        self.node_height
    }

    pub fn node_margin_x(&self) -> f64 {
        self.node_margin_x
    }

    pub fn node_margin_y(&self) -> f64 {
        self.node_margin_y
    }

    pub fn plain_text_color(&self) -> &str {
        self.plain_text_color
    }

    pub fn emoji_point_size(&self) -> u32 {
        self.emoji_point_size
    }

    pub fn node_point_size(&self) -> u32 {
        self.node_point_size
    }

    pub fn tag_width(&self) -> f64 {
        self.tag_width
    }

    pub fn tag_height(&self) -> f64 {
        self.tag_height
    }

    pub fn tag_margin_x(&self) -> f64 {
        self.tag_margin_x
    }

    pub fn tag_margin_y(&self) -> f64 {
        self.tag_margin_y
    }

    pub fn tag_point_size(&self) -> u32 {
        self.tag_point_size
    }

    pub fn tag_classes(&self) -> &'static str {
        self.tag_classes
    }
}

impl Default for GraphvizDotTheme {
    fn default() -> Self {
        Self {
            edge_color: "#333333",
            node_text_color: "#111111",
            node_width: 0.3,
            node_height: 0.1,
            node_margin_x: 0.04,
            node_margin_y: 0.04,
            plain_text_color: "#222222",
            emoji_point_size: 14,
            node_point_size: 10,
            tag_width: 0.3,
            tag_height: 0.1,
            tag_margin_x: 0.03,
            tag_margin_y: 0.02,
            tag_point_size: 8,
            tag_classes: "\
                [&>path]:fill-emerald-200 \
                [&>path]:stroke-emerald-500 \
                [&>path]:hover:fill-emerald-100 \
                [&>path]:hover:stroke-emerald-400 \
                [&>path]:focus:fill-lime-200 \
                [&>path]:focus:outline-1 \
                [&>path]:focus:outline-lime-600 \
                [&>path]:focus:outline-dashed \
                [&>path]:focus:rounded-xl \
                cursor-pointer \
            ",
        }
    }
}
