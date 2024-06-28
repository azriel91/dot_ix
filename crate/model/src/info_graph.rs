use std::collections::{HashMap, VecDeque};

pub use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::{
        EdgeDescs, Edges, GraphvizAttrs, NodeDescs, NodeEmojis, NodeHierarchy, NodeId, NodeNames,
        NodeTags, TagId,
    },
    theme::Theme,
};

pub use self::{graph_dir::GraphDir, info_graph_builder::InfoGraphBuilder, tag::Tag};

mod graph_dir;
mod info_graph_builder;
mod tag;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoGraph {
    /// Direction of the graph, `vertical` or `horizontal`.
    pub(crate) direction: GraphDir,
    /// Nested nodes.
    pub(crate) hierarchy: NodeHierarchy,
    /// Each node's name.
    pub(crate) node_names: NodeNames,
    /// Each node's description.
    pub(crate) node_descs: NodeDescs,
    /// Each node's emoji.
    pub(crate) node_emojis: NodeEmojis,
    /// Tags associated with each node.
    pub(crate) node_tags: NodeTags,
    /// Logical / ordering dependencies.
    pub(crate) edges: Edges,
    /// Each edge's description.
    pub(crate) edge_descs: EdgeDescs,
    /// Tags to associate with nodes.
    pub(crate) tags: IndexMap<TagId, Tag>,
    /// Additional attributes specifically for GraphViz.
    pub(crate) graphviz_attrs: GraphvizAttrs,
    /// Theme that controls the CSS classes to add to elements.
    pub(crate) theme: Theme,
    /// Additional CSS to add in the spreadsheet.
    pub(crate) css: String,
}

impl InfoGraph {
    /// Returns a builder to instantiate an `InfoGraph`.
    pub fn builder() -> InfoGraphBuilder {
        InfoGraphBuilder::default()
    }

    /// Returns the direction of the graph, `vertical` or `horizontal`.
    pub fn direction(&self) -> GraphDir {
        self.direction
    }

    /// Returns the nested nodes.
    pub fn hierarchy(&self) -> &NodeHierarchy {
        &self.hierarchy
    }

    /// Returns a flatten map of the node hierarchy.
    ///
    /// For example, if the hierarchy is:
    ///
    /// ```yaml
    /// a:
    ///   b:
    ///     c: {}
    ///     d: {}
    /// ```
    ///
    /// This returns:
    ///
    /// ```yaml
    /// a:
    ///   b: {}
    /// b:
    ///   c: {}
    ///   d: {}
    /// c: {}
    /// d: {}
    /// ```
    pub fn hierarchy_flat(&self) -> HashMap<&NodeId, &NodeHierarchy> {
        let mut node_id_to_hierarchy =
            HashMap::<&NodeId, &NodeHierarchy>::with_capacity(self.edges().len());
        let mut hierarchy_queue = VecDeque::new();
        hierarchy_queue.push_back(self.hierarchy());

        while let Some(hierarchy) = hierarchy_queue.pop_front() {
            hierarchy.iter().for_each(|(node_id, node_hierarchy)| {
                node_id_to_hierarchy.insert(node_id, node_hierarchy);
                hierarchy_queue.push_back(node_hierarchy);
            });
        }

        node_id_to_hierarchy
    }

    /// Returns the map of node names.
    pub fn node_names(&self) -> &NodeNames {
        &self.node_names
    }

    /// Returns the map of node descriptions.
    pub fn node_descs(&self) -> &NodeDescs {
        &self.node_descs
    }

    /// Returns the map of node emojis.
    pub fn node_emojis(&self) -> &NodeEmojis {
        &self.node_emojis
    }

    /// Returns the tags associated with each node.
    pub fn node_tags(&self) -> &NodeTags {
        &self.node_tags
    }

    /// Returns the logical / ordering dependencies.
    pub fn edges(&self) -> &Edges {
        &self.edges
    }

    /// Returns the map of edge descriptions.
    pub fn edge_descs(&self) -> &EdgeDescs {
        &self.edge_descs
    }

    /// Returns the tags to associate with nodes.
    pub fn tags(&self) -> &IndexMap<TagId, Tag> {
        &self.tags
    }

    /// Returns the additional attributes specifically for GraphViz.
    pub fn graphviz_attrs(&self) -> &GraphvizAttrs {
        &self.graphviz_attrs
    }

    /// Returns the theme that controls the CSS classes to add to elements.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns the additional CSS to add in the spreadsheet.
    pub fn css(&self) -> &str {
        &self.css
    }
}
