use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::model::common::{NodeId, TailwindKey};

/// Map of tailwind keys to tailwind classes to apply for that key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TailwindClasses(IndexMap<TailwindKey, String>);

impl TailwindClasses {
    /// Returns a new `TailwindClasses` map without any allocation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `TailwindClasses` map with the given initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    /// Returns the underlying `IndexMap`.
    pub fn into_inner(self) -> IndexMap<TailwindKey, String> {
        self.0
    }

    /// Returns the default node styling to use.
    ///
    /// In order of precedence:
    ///
    /// * `node_defaults` from deserialized value.
    /// * Hard coded defaults.
    pub fn node_defaults(&self) -> &str {
        self.0
            .get(&TailwindKey::NodeDefaults)
            .map(String::as_str)
            .unwrap_or_else(|| {
                "\
                [&>path]:fill-slate-300 \
                [&>path]:stroke-1 \
                [&>path]:stroke-slate-600 \
                [&>path]:hover:fill-slate-200 \
                [&>path]:hover:stroke-slate-600 \
                [&>path]:focus:fill-lime-200 \
                [&>path]:focus:outline-1 \
                [&>path]:focus:outline-lime-600 \
                [&>path]:focus:outline-dashed \
                [&>path]:focus:rounded-xl \
                cursor-pointer \
            "
                .trim()
            })
    }

    /// Returns the default node styling to use.
    ///
    /// In order of precedence:
    ///
    /// * `node_defaults` from deserialized value.
    /// * Hard coded defaults.
    pub fn node_classes_or_default(&self, node_id: NodeId) -> &str {
        self.0
            .get(&TailwindKey::AnyId(node_id.into()))
            .map(String::as_str)
            .unwrap_or_else(|| self.node_defaults())
    }

    /// Returns the default edge styling to use.
    ///
    /// In order of precedence:
    ///
    /// * `edge_defaults` from deserialized value.
    /// * Hard coded defaults.
    pub fn edge_defaults(&self) -> &str {
        self.0
            .get(&TailwindKey::NodeDefaults)
            .map(String::as_str)
            .unwrap_or_else(|| "".trim())
    }
}

impl Deref for TailwindClasses {
    type Target = IndexMap<TailwindKey, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TailwindClasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
