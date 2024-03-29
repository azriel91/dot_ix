use dot_ix::model::common::Edge;

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
