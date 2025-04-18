//! Model for the `dot_ix` diagramming application.
//!
//! # Design
//!
//! There are two paradigms considered when structuring the serialized data:
//! array of structs (AOS) and struct of arrays (SOA).
//!
//! The subsections below show what each of these structurings *could* look like
//! when serialized.
//!
//! The [`soa_derive`] crate provides the `StructOfArray` derive macro to
//! generate the struct of arrays data structure from a object-style struct.
//! However, it only works if the serialized data is of equal length for every
//! attribute -- indices must match exactly.
//!
//! We can manually implement a balance between the AOS and SOA styles for what
//! makes sense to the user:
//!
//! * Include the most important attributes in the main node declaration.
//! * Use separate attribute maps for different attributes that are occasionally
//!   present.
//! * The richness of data in one attribute map depends on the "data proximity"
//!   -- if information happens together, then group them into one attribute
//!   map.
//!
//! [`soa_derive`]: https://crates.io/crates/soa_derive
//!
//! ## Array of Structs
//!
//! ```yaml
//! nodes:
//! - node_a:
//!   name: "Node A"
//!   desc: Contains things to do with A.
//!   children:
//!   - node_a0:
//!     name: "A Child 0"
//!   - node_a1:
//!     name: "A Child 1"
//! - node_b:
//!   name: "Node B"
//!   children:
//!   - node_b0:
//!     name: "B Child 0"
//! edges:
//! - edge_a_b: [node_a, node_b]
//! - edge_a1_b0: [node_a1, node_b0]
//! ```
//!
//! ### Attributes
//!
//! * All data for a node is within that node.
//! * Information for one node is grouped together -- easier to manage different
//!   information for one node.
//! * Noisy when managing one kind of information across nodes.
//!
//! ## Struct of Arrays
//!
//! ```yaml
//! nodes:
//! - node_a
//! - node_a0
//! - node_a1
//! - node_b
//! - node_b0
//!
//! edges:
//!   edge_a_b: [node_a, node_b]
//!   edge_a1_b0: [node_a1, node_b0]
//!
//! children:
//!   node_a: [node_a0, node_a1]
//!   node_b: [node_b0]
//!
//! node_names:
//!   node_a : "Node A"
//!   node_a0: "A Child 0"
//!   node_a1: "A Child 1"
//!   node_b : "Node B"
//!   node_b0: "B Child 0"
//!
//! node_descs:
//!   node_a: Contains things to do with A.
//! ```
//!
//! ### Attributes
//!
//! * Data for a node is split across information kinds.
//! * Information for one node is spread across different locations -- harder to
//!   manage different information for one node.
//! * One kind of information across nodes is grouped together -- easier to
//!   manage one kind of information for multiple nodes.
//!
//!
//! ## Manual Balanced Approach
//!
//! ```yaml
//! hierarchy:
//!   node_a:
//!     node_a0: {}
//!     node_a1: {}
//!   node_b:
//!     node_b0: {}
//!
//! edges:
//!   edge_a_b: [node_a, node_b]
//!   edge_a1_b0: [node_a1, node_b0]
//!
//! node_infos:
//!   node_a:
//!     name: "⚙️ Node A"
//!     desc: Contains things to do with A.
//!   node_a0: "A Child 0" # shorthand
//!   node_a1: "A Child 1"
//!   node_b : "Node B"
//!   node_b0: "B Child 0"
//!
//! node_tags:
//!   node_a: [tag_0, tag_1]
//!   node_a0: [tag_0]
//!   node_a1: [tag_1]
//!   node_b: [tag_0]
//!   node_b0: [tag_0]
//!
//! # tags are not necessarily associated with a node.
//! tags:
//! - tag_0: { name: "Tag 0", desc: "Some information for tag 0." }
//! - tag_1: "Tag 1"
//! - tag_2: "Tag 2"
//! ```

#[macro_use]
extern crate id_newtype;

// Re-exports
pub use dot_ix_static_check_macros::{edge_id, node_id, tag_id};
pub use indexmap::IndexMap;

pub mod common;
pub mod info_graph;
pub mod theme;
