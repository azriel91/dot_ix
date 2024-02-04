use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::NodeId;

/// A node's child nodes -- this doesn't include itself.
///
/// The root hierarchy is on the graph type you are using, e.g. [`InfoGraph`].
///
/// [`InfoGraph`]: crate::info_graph::InfoGraph
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeHierarchy(IndexMap<NodeId, NodeHierarchy>);

impl NodeHierarchy {
    /// Creates an empty `NodeHierarchy` map.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dot_ix::model::common::NodeHierarchy;
    /// let node_hierarchy = NodeHierarchy::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `NodeHierarchy` map with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dot_ix::model::common::NodeHierarchy;
    /// let node_hierarchy: NodeHierarchy = NodeHierarchy::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `NodeHierarchy<K, V>` might be able to
    /// hold more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dot_ix::model::common::NodeHierarchy;
    /// let node_hierarchy: NodeHierarchy = NodeHierarchy::with_capacity(100);
    /// assert!(node_hierarchy.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> IndexMap<NodeId, NodeHierarchy> {
        self.0
    }
}

impl std::ops::Deref for NodeHierarchy {
    type Target = IndexMap<NodeId, NodeHierarchy>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NodeHierarchy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
