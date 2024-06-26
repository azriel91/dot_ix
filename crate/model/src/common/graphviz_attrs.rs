use serde::{Deserialize, Serialize};

pub use self::{edge_constraints::EdgeConstraints, edge_minlens::EdgeMinlens};

mod edge_constraints;
mod edge_minlens;

/// Additional attributes specifically for GraphViz.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct GraphvizAttrs {
    /// The default constraint value for edges, defaults to `true`.
    pub edge_constraint_default: bool,
    /// Each edge's constraints.
    pub edge_constraints: EdgeConstraints,
    /// The default [`minlen`] for edges, defaults to `1`.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub edge_minlen_default: u32,
    /// Each edge's [`minlen`].
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub edge_minlens: EdgeMinlens,
}

impl GraphvizAttrs {
    /// Returns a new `GraphvizOpts` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the default constraint value for edges.
    pub fn edge_constraint_default(&self) -> bool {
        self.edge_constraint_default
    }

    /// Returns the map of edge constraints.
    pub fn edge_constraints(&self) -> &EdgeConstraints {
        &self.edge_constraints
    }

    /// Returns the default `minlen` value for edges.
    pub fn edge_minlen_default(&self) -> u32 {
        self.edge_minlen_default
    }

    /// Returns the map of edge `minlen`s.
    pub fn edge_minlens(&self) -> &EdgeMinlens {
        &self.edge_minlens
    }
}

impl Default for GraphvizAttrs {
    fn default() -> Self {
        Self {
            edge_constraint_default: true,
            edge_constraints: EdgeConstraints::default(),
            edge_minlen_default: 1,
            edge_minlens: EdgeMinlens::default(),
        }
    }
}
