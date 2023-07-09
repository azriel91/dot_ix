#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphvizDotTheme {
    edge_color: &'static str,
    node_text_color: &'static str,
    plain_text_color: &'static str,
}

impl GraphvizDotTheme {
    pub fn new(
        edge_color: &'static str,
        node_text_color: &'static str,
        plain_text_color: &'static str,
    ) -> Self {
        Self {
            edge_color,
            node_text_color,
            plain_text_color,
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
}

impl Default for GraphvizDotTheme {
    fn default() -> Self {
        Self {
            edge_color: "#7f7f7f",
            node_text_color: "#111111",
            plain_text_color: "#7f7f7f",
        }
    }
}
