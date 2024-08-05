use dot_ix::{
    model::{
        node_id, tag_id,
        theme::{AnyIdOrDefaults, CssClassPartials, CssClasses, Theme, ThemeAttr},
    },
    rt::InfoGraphDot,
};

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

#[test]
fn tag_theme_merge_resolves_node_outline() {
    let mut tag_theme = Theme::new();
    tag_theme.insert(
        AnyIdOrDefaults::NodeDefaults,
        CssClassPartials::from_iter([
            (ThemeAttr::OutlineColor, "red".to_string()),
            (ThemeAttr::OutlineStyle, "dashed".to_string()),
            (ThemeAttr::OutlineWidth, "[2px]".to_string()),
            (ThemeAttr::OutlineShade, "600".to_string()),
        ]),
    );
    let test_node_id = node_id!("my_node");
    let diagram_theme = Theme::new();
    let info_graph_dot = InfoGraphDot {
        node_ids: vec![&test_node_id],
        edge_ids: vec![],
    };
    let themeable = &info_graph_dot;

    let (tag_el_css_classes, _theme_warnings) =
        tag_theme.tag_el_css_classes(themeable, &diagram_theme, &tag_id!("tag_step_1"));

    let css_classes = tag_el_css_classes.get(test_node_id.as_str());
    assert_eq!(
        Some(CssClasses::from(
            "\
                peer-focus/tag_step_1:[&>path]:outline-red-600 \
                peer-focus/tag_step_1:[&>ellipse]:outline-red-600 \
                peer-focus/tag_step_1:[&>path]:outline-[2px] \
                peer-focus/tag_step_1:[&>path]:outline-dashed \
                peer-focus/tag_step_1:[&>ellipse]:outline-[2px] \
                peer-focus/tag_step_1:[&>ellipse]:outline-dashed \
                peer-focus/tag_step_1:[&>path]:focus:outline-red-600 \
                peer-focus/tag_step_1:[&>ellipse]:focus:outline-red-600 \
                peer-focus/tag_step_1:[&>path]:focus:outline-[2px] \
                peer-focus/tag_step_1:[&>path]:focus:outline-dashed \
                peer-focus/tag_step_1:[&>ellipse]:focus:outline-[2px] \
                peer-focus/tag_step_1:[&>ellipse]:focus:outline-dashed \
                peer-focus/tag_step_1:[&>path]:hover:outline-red-600 \
                peer-focus/tag_step_1:[&>ellipse]:hover:outline-red-600 \
                peer-focus/tag_step_1:[&>path]:hover:outline-[2px] \
                peer-focus/tag_step_1:[&>path]:hover:outline-dashed \
                peer-focus/tag_step_1:[&>ellipse]:hover:outline-[2px] \
                peer-focus/tag_step_1:[&>ellipse]:hover:outline-dashed \
                peer-focus/tag_step_1:[&>path]:focus:active:outline-red-600 \
                peer-focus/tag_step_1:[&>ellipse]:focus:active:outline-red-600 \
                peer-focus/tag_step_1:[&>path]:focus:active:outline-[2px] \
                peer-focus/tag_step_1:[&>path]:focus:active:outline-dashed \
                peer-focus/tag_step_1:[&>ellipse]:focus:active:outline-[2px] \
                peer-focus/tag_step_1:[&>ellipse]:focus:active:outline-dashed \
            "
            .to_string()
        ))
        .as_ref(),
        css_classes
    );
}
