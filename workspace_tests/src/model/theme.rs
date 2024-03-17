use dot_ix::model::theme::{AnyIdOrDefaults, Theme, ThemeAttr};

#[test]
fn theme_default_is_empty() {
    let theme = Theme::default();

    assert!(theme.is_empty())
}

#[test]
fn base_theme_contains_node_defaults_and_edge_defaults() {
    let theme = Theme::base();

    let node_defaults = theme.get(&AnyIdOrDefaults::NodeDefaults);
    let edge_defaults = theme.get(&AnyIdOrDefaults::EdgeDefaults);

    assert!(node_defaults.is_some());
    let node_defaults = node_defaults.expect("Expected `node_defaults` to exist.");
    assert!(node_defaults.get(&ThemeAttr::Padding).is_some());
    assert!(node_defaults.get(&ThemeAttr::ShapeColor).is_some());
    assert!(node_defaults.get(&ThemeAttr::FillShadeNormal).is_some());
    assert!(node_defaults.get(&ThemeAttr::FillShadeFocus).is_some());
    assert!(node_defaults.get(&ThemeAttr::FillShadeHover).is_some());
    assert!(node_defaults.get(&ThemeAttr::FillShadeActive).is_some());
    assert!(node_defaults.get(&ThemeAttr::StrokeShadeNormal).is_some());
    assert!(node_defaults.get(&ThemeAttr::StrokeShadeFocus).is_some());
    assert!(node_defaults.get(&ThemeAttr::StrokeShadeHover).is_some());
    assert!(node_defaults.get(&ThemeAttr::StrokeShadeActive).is_some());

    assert!(edge_defaults.is_some());
    let edge_defaults = edge_defaults.expect("Expected `edge_defaults` to exist.");
    assert!(edge_defaults.get(&ThemeAttr::ShapeColor).is_some());
    assert!(edge_defaults.get(&ThemeAttr::FillShadeNormal).is_some());
    assert!(edge_defaults.get(&ThemeAttr::FillShadeFocus).is_some());
    assert!(edge_defaults.get(&ThemeAttr::FillShadeHover).is_some());
    assert!(edge_defaults.get(&ThemeAttr::FillShadeActive).is_some());
    assert!(edge_defaults.get(&ThemeAttr::StrokeShadeNormal).is_some());
    assert!(edge_defaults.get(&ThemeAttr::StrokeShadeFocus).is_some());
    assert!(edge_defaults.get(&ThemeAttr::StrokeShadeHover).is_some());
    assert!(edge_defaults.get(&ThemeAttr::StrokeShadeActive).is_some());

    assert_eq!(2, theme.len());
}
