use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::EdgeId;

pub use self::edge_constraints::EdgeConstraints;

mod edge_constraints;

/// Additional attributes specifically for GraphViz.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GraphvizAttrs {
    /// The default constraint value for edges, defaults to `true`.
    pub edge_constraint_default: bool,
    /// Each edge's constraints.
    pub edge_constraints: EdgeConstraints,
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
}

impl Default for GraphvizAttrs {
    fn default() -> Self {
        Self {
            edge_constraint_default: true,
            edge_constraints: EdgeConstraints::default(),
        }
    }
}
