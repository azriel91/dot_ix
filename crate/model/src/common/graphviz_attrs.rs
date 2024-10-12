use serde::{Deserialize, Serialize};

pub use self::{
    edge_constraints::EdgeConstraints, edge_dir::EdgeDir, edge_dirs::EdgeDirs,
    edge_minlens::EdgeMinlens, fixed_size::FixedSize, node_heights::NodeHeights,
    node_widths::NodeWidths, pack_mode::PackMode, pack_mode_flag::PackModeFlag, splines::Splines,
};

mod edge_constraints;
mod edge_dir;
mod edge_dirs;
mod edge_minlens;
mod fixed_size;
mod node_heights;
mod node_widths;
mod pack_mode;
mod pack_mode_flag;
mod splines;

/// Additional attributes specifically for GraphViz.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct GraphvizAttrs {
    /// Minimum space between two adjacent nodes in the same rank, in
    /// inches. Also controls the spacing between multiple edges between the
    /// same pair of nodes.
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// See [`nodesep`].
    ///
    /// [`nodesep`]: https://graphviz.org/docs/attrs/nodesep/
    pub nodesep: f64,
    /// The desired separation between nodes of different ranks, in inches. See
    /// [`ranksep`].
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// This does not support the `equally` string (yet). I'm not sure if it is
    /// used.
    ///
    /// [`ranksep`]: https://graphviz.org/docs/attrs/ranksep/
    pub ranksep: f64,
    /// How to render edge lines. See [`splines`].
    ///
    /// [`splines`]: https://graphviz.org/docs/attrs/splines/
    pub splines: Splines,
    /// The default [`constraint`] value for edges, defaults to `true`.
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub edge_constraint_default: bool,
    /// Each edge's [`constraint`].
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub edge_constraints: EdgeConstraints,
    /// The default [`dir`] value for edges, defaults to [`EdgeDir::Forward`].
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub edge_dir_default: EdgeDir,
    /// Each edge's [`dir`].
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub edge_dirs: EdgeDirs,
    /// The default [`minlen`] for edges, defaults to `1`.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub edge_minlen_default: u32,
    /// Each edge's [`minlen`].
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub edge_minlens: EdgeMinlens,
    /// Minimum / initial [`width`] for nodes, defaults to `0.3`.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub node_width_default: f64,
    /// Each node's [`width`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub node_widths: NodeWidths,
    /// Minimum / initial [`height`] for nodes, defaults to `0.1`.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub node_height_default: f64,
    /// Each node's [`height`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub node_heights: NodeHeights,
    /// Whether a node's `width` and `height` are fixed dimensions.
    ///
    /// See [`fixedsize`].
    ///
    /// [`fixedsize`]: https://graphviz.org/docs/attrs/fixedsize/
    pub fixed_size: FixedSize,
    /// How closely to pack together graph components.
    pub pack_mode: PackMode,
}

impl GraphvizAttrs {
    /// Returns a new `GraphvizOpts` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum space between two adjacent nodes in the same rank, in
    /// inches. Also controls the spacing between multiple edges between the
    /// same pair of nodes.
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// See [`nodesep`].
    ///
    /// [`nodesep`]: https://graphviz.org/docs/attrs/nodesep/
    pub fn with_nodesep(mut self, nodesep: f64) -> Self {
        self.nodesep = nodesep;
        self
    }

    /// Sets the desired separation between nodes of different ranks, in inches.
    /// See [`ranksep`].
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// This does not support the `equally` string (yet). I'm not sure if it is
    /// used.
    ///
    /// [`ranksep`]: https://graphviz.org/docs/attrs/ranksep/
    pub fn with_ranksep(mut self, ranksep: f64) -> Self {
        self.ranksep = ranksep;
        self
    }

    /// Sets how to render edge lines. See [`splines`].
    ///
    /// [`splines`]: https://graphviz.org/docs/attrs/splines/
    pub fn with_splines(mut self, splines: Splines) -> Self {
        self.splines = splines;
        self
    }

    /// Sets the default [`constraint`] value for edges.
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub fn with_edge_constraint_default(mut self, edge_constraint_default: bool) -> Self {
        self.edge_constraint_default = edge_constraint_default;
        self
    }

    /// Sets the map of edge [`constraint`]s.
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub fn with_edge_constraints(mut self, edge_constraints: EdgeConstraints) -> Self {
        self.edge_constraints = edge_constraints;
        self
    }

    /// Sets the default [`dir`] value for edges.
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub fn with_edge_dir_default(mut self, edge_dir_default: EdgeDir) -> Self {
        self.edge_dir_default = edge_dir_default;
        self
    }

    /// Sets the map of edge [`dir`]s.
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub fn with_edge_dirs(mut self, edge_dirs: EdgeDirs) -> Self {
        self.edge_dirs = edge_dirs;
        self
    }

    /// Sets the default [`minlen`] value for edges.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub fn with_edge_minlen_default(mut self, edge_minlen_default: u32) -> Self {
        self.edge_minlen_default = edge_minlen_default;
        self
    }

    /// Sets the map of edge [`minlen`]s.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub fn with_edge_minlens(mut self, edge_minlens: EdgeMinlens) -> Self {
        self.edge_minlens = edge_minlens;
        self
    }

    /// Sets the minimum / initial [`width`] for nodes, defaults to `0.3`.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub fn with_node_width_default(mut self, node_width_default: f64) -> Self {
        self.node_width_default = node_width_default;
        self
    }

    /// Sets each node's [`width`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub fn with_node_widths(mut self, node_widths: NodeWidths) -> Self {
        self.node_widths = node_widths;
        self
    }

    /// Sets the minimum / initial [`height`] for nodes, defaults to `0.1`.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub fn with_node_height_default(mut self, node_height_default: f64) -> Self {
        self.node_height_default = node_height_default;
        self
    }

    /// Sets each node's [`height`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub fn with_node_heights(mut self, node_heights: NodeHeights) -> Self {
        self.node_heights = node_heights;
        self
    }

    /// Sets whether a node's `width` and `height` are fixed dimensions.
    ///
    /// See [`fixedsize`].
    ///
    /// [`fixedsize`]: https://graphviz.org/docs/attrs/fixedsize/
    pub fn with_fixed_size(mut self, fixed_size: FixedSize) -> Self {
        self.fixed_size = fixed_size;
        self
    }

    /// Returns the minimum space between two adjacent nodes in the same rank,
    /// in inches. Also controls the spacing between multiple edges between
    /// the same pair of nodes.
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// See [`nodesep`].
    ///
    /// [`nodesep`]: https://graphviz.org/docs/attrs/nodesep/
    pub fn nodesep(&self) -> f64 {
        self.nodesep
    }

    /// Returns the desired separation between nodes of different ranks, in
    /// inches. See [`ranksep`].
    ///
    /// Defaults to `0.25`. Minimum `0.02`.
    ///
    /// [`ranksep`]: https://graphviz.org/docs/attrs/ranksep/
    pub fn ranksep(&self) -> f64 {
        self.ranksep
    }

    /// Returns how to render edge lines. See [`splines`].
    ///
    /// [`splines`]: https://graphviz.org/docs/attrs/splines/
    pub fn splines(&self) -> Splines {
        self.splines
    }

    /// Returns the default [`constraint`] value for edges.
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub fn edge_constraint_default(&self) -> bool {
        self.edge_constraint_default
    }

    /// Returns the map of edge [`constraint`]s.
    ///
    /// [`constraint`]: https://graphviz.org/docs/attrs/constraint/
    pub fn edge_constraints(&self) -> &EdgeConstraints {
        &self.edge_constraints
    }

    /// Returns the default [`dir`] value for edges.
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub fn edge_dir_default(&self) -> EdgeDir {
        self.edge_dir_default
    }

    /// Returns the map of edge [`dir`]s.
    ///
    /// [`dir`]: https://graphviz.org/docs/attrs/dir/
    pub fn edge_dirs(&self) -> &EdgeDirs {
        &self.edge_dirs
    }

    /// Returns the default [`minlen`] value for edges.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub fn edge_minlen_default(&self) -> u32 {
        self.edge_minlen_default
    }

    /// Returns the map of edge [`minlen`]s.
    ///
    /// [`minlen`]: https://graphviz.org/docs/attrs/minlen/
    pub fn edge_minlens(&self) -> &EdgeMinlens {
        &self.edge_minlens
    }

    /// Returns the minimum / initial [`width`] for nodes.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub fn node_width_default(&self) -> f64 {
        self.node_width_default
    }

    /// Returns each node's [`width`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum width for
    /// nodes.
    ///
    /// [`width`]: https://graphviz.org/docs/attrs/width/
    pub fn node_widths(&self) -> &NodeWidths {
        &self.node_widths
    }

    /// Returns the minimum / initial [`height`] for nodes.
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub fn node_height_default(&self) -> f64 {
        self.node_height_default
    }

    /// Returns each node's [`height`].
    ///
    /// If `fixedsize` is true, this will be the exact / maximum height for
    /// nodes.
    ///
    /// [`height`]: https://graphviz.org/docs/attrs/height/
    pub fn node_heights(&self) -> &NodeHeights {
        &self.node_heights
    }

    /// Returns whether a node's `width` and `height` are fixed dimensions.
    ///
    /// See [`fixedsize`].
    ///
    /// [`fixedsize`]: https://graphviz.org/docs/attrs/fixedsize/
    pub fn fixed_size(&self) -> FixedSize {
        self.fixed_size
    }
}

impl Default for GraphvizAttrs {
    fn default() -> Self {
        Self {
            nodesep: 0.25,
            ranksep: 0.25,
            splines: Splines::default(),
            edge_constraint_default: true,
            edge_constraints: EdgeConstraints::default(),
            edge_dir_default: EdgeDir::default(),
            edge_dirs: EdgeDirs::default(),
            edge_minlen_default: 2,
            edge_minlens: EdgeMinlens::default(),
            node_width_default: 0.3,
            node_widths: NodeWidths::default(),
            node_height_default: 0.1,
            node_heights: NodeHeights::default(),
            fixed_size: FixedSize::default(),
            pack_mode: PackMode::default(),
        }
    }
}
