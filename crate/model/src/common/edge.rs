/// Edge between two functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// Logical / functional dependency.
    ///
    /// The predecessor node points to the successor node.
    Logic,
    /// The predecessor node contains the successor node.
    Contains,
    /// This edge is purely for adjusting how `dot` lays out the graph.
    Layout,
}

#[cfg(test)]
mod tests {
    use super::Edge;

    #[test]
    fn clone() {
        let edge = Edge::Logic;

        assert_eq!(Edge::Logic, edge.clone());
    }

    #[test]
    fn debug() {
        let edge = Edge::Logic;

        assert_eq!("Logic", format!("{edge:?}"));
    }
}
