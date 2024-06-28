#[derive(Clone, Debug, PartialEq)]
pub struct GraphvizDotTheme {
    /// The colour to use for graph edges.
    pub edge_color: &'static str,

    // Node attributes:
    //
    // <https://graphviz.org/docs/nodes/>
    pub node_text_color: &'static str,
    /// Width of a node, but it is allowed to expand.
    pub node_width: f64,
    /// Height of a node, but it is allowed to expand.
    pub node_height: f64,
    /// Left and right margin in inches.
    ///
    /// Default: `0.04`.
    ///
    /// Graphviz default: `0.11`.
    pub node_margin_x: f64,
    /// Top and bottom margin in inches.
    ///
    /// Default: `0.04`.
    ///
    /// Graphviz default: `0.055`.
    pub node_margin_y: f64,
    pub plain_text_color: &'static str,
    pub emoji_point_size: u32,
    /// Default font point size for node labels.
    ///
    /// Default: `10`
    pub node_point_size: u32,
    /// Default font point size for edge labels.
    ///
    /// Default: `10`
    pub edge_point_size: u32,
    pub tag_width: f64,
    pub tag_height: f64,
    /// Left and right margin in inches.
    ///
    /// Default: `0.03`.
    ///
    /// Graphviz default: `0.11`.
    pub tag_margin_x: f64,
    /// Top and bottom margin in inches.
    ///
    /// Default: `0.02`.
    ///
    /// Graphviz default: `0.055`.
    pub tag_margin_y: f64,
    pub tag_point_size: u32,
    pub tag_classes: &'static str,
}

impl GraphvizDotTheme {
    pub fn with_edge_color(mut self, edge_color: &'static str) -> Self {
        self.edge_color = edge_color;
        self
    }

    pub fn with_node_text_color(mut self, node_text_color: &'static str) -> Self {
        self.node_text_color = node_text_color;
        self
    }

    pub fn with_node_width(mut self, node_width: f64) -> Self {
        self.node_width = node_width;
        self
    }

    pub fn with_node_height(mut self, node_height: f64) -> Self {
        self.node_height = node_height;
        self
    }

    pub fn with_node_margin_x(mut self, node_margin_x: f64) -> Self {
        self.node_margin_x = node_margin_x;
        self
    }

    pub fn with_node_margin_y(mut self, node_margin_y: f64) -> Self {
        self.node_margin_y = node_margin_y;
        self
    }

    pub fn with_plain_text_color(mut self, plain_text_color: &'static str) -> Self {
        self.plain_text_color = plain_text_color;
        self
    }

    pub fn with_emoji_point_size(mut self, emoji_point_size: u32) -> Self {
        self.emoji_point_size = emoji_point_size;
        self
    }

    pub fn with_node_point_size(mut self, node_point_size: u32) -> Self {
        self.node_point_size = node_point_size;
        self
    }

    pub fn with_edge_point_size(mut self, edge_point_size: u32) -> Self {
        self.edge_point_size = edge_point_size;
        self
    }

    pub fn with_tag_width(mut self, tag_width: f64) -> Self {
        self.tag_width = tag_width;
        self
    }

    pub fn with_tag_height(mut self, tag_height: f64) -> Self {
        self.tag_height = tag_height;
        self
    }

    pub fn with_tag_margin_x(mut self, tag_margin_x: f64) -> Self {
        self.tag_margin_x = tag_margin_x;
        self
    }

    pub fn with_tag_margin_y(mut self, tag_margin_y: f64) -> Self {
        self.tag_margin_y = tag_margin_y;
        self
    }

    pub fn with_tag_point_size(mut self, tag_point_size: u32) -> Self {
        self.tag_point_size = tag_point_size;
        self
    }

    pub fn with_tag_classes(mut self, tag_classes: &'static str) -> Self {
        self.tag_classes = tag_classes;
        self
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

    pub fn edge_point_size(&self) -> u32 {
        self.edge_point_size
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
            edge_point_size: 10,
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
