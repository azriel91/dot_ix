#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphvizDotTheme {
    edge_color: &'static str,
    node_text_color: &'static str,
    plain_text_color: &'static str,
    emoji_point_size: u32,
    node_point_size: u32,
}

impl GraphvizDotTheme {
    pub fn new(
        edge_color: &'static str,
        node_text_color: &'static str,
        plain_text_color: &'static str,
        emoji_point_size: u32,
        node_point_size: u32,
    ) -> Self {
        Self {
            edge_color,
            node_text_color,
            plain_text_color,
            emoji_point_size,
            node_point_size,
        }
    }

    pub fn edge_color(&self) -> &str {
        self.edge_color
    }

    pub fn node_text_color(&self) -> &str {
        self.node_text_color
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
}

impl Default for GraphvizDotTheme {
    fn default() -> Self {
        Self {
            edge_color: "#7f7f7f",
            node_text_color: "#111111",
            plain_text_color: "#7f7f7f",
            emoji_point_size: 14,
            node_point_size: 10,
        }
    }
}
