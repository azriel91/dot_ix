# Changelog

## 0.9.1 (2025-01-11)

* Update dependency versions.


## 0.9.0 (2024-12-15)

* ***Breaking:*** Upgrade to `leptos 0.7.0`
* Support `margins` in `GraphvizAttrs`.
* Support `margin_cluster_default` in `GraphvizAttrs`.
* Support `margin_node_default` in `GraphvizAttrs`.
* Support `node_width_default` in `GraphvizAttrs`.
* Support `node_widths` in `GraphvizAttrs`.
* Support `node_height_default` in `GraphvizAttrs`.
* Support `node_heights` in `GraphvizAttrs`.
* Support `fixed_size` in `GraphvizAttrs`.


## 0.8.1 (2024-09-27)

* Support inline images ([#33]).
* Support `splines` in `GraphvizAttrs`.
* Support `nodesep` in `GraphvizAttrs`.
* Support `ranksep` in `GraphvizAttrs`.


[#33]: https://github.com/azriel91/dot_ix/issues/33


## 0.8.0 (2024-08-08)

* Add `TextEditor` in playground which uses [monaco][monaco] / [rust-monaco][rust-monaco].
* Add `GraphvizAttrs.pack_mode` to specify the `packmode` for subgraphs.
* Fix leading space not rendered in web SVG view.
* Add `ThemeAttr::Cursor` for better support for [cursor styling].
* Support `dasharray:5,2,3,2..` in `stroke_style` in SVG.
* Support stroke style for ellipse elements in SVG.
* Reimplement playground for better responsive layout.
* Take in `svg_extra` elements and insert into generated SVG.
* Allow cluster edge ports to work correctly.
* Take in `tags` which only include tag names.
* Take in `tag_items` for nodes and edges instead of tags per node.
* Support theming nodes based on tag focus.
* Add `ThemeAttr::Animate` to support setting [`animation`].
* Add `ThemeAttr::Visibility` to support setting [`visibility`].
* Fix black outline shown on focused nodes in Chrome / Edge.
* Show feedback to user when stroke / outline / fill class partials are not all specified.
* Add examples to playground.

[monaco]: https://github.com/microsoft/monaco-editor
[rust-monaco]: https://github.com/siku2/rust-monaco
[cursor styling]: https://tailwindcss.com/docs/cursor
[`animation`]: https://tailwindcss.com/docs/animation
[`visibility`]: https://tailwindcss.com/docs/visibility


## 0.7.0 (2024-06-30)

* Playground: Use URL fragment/hash instead of query params to store source.
* Make `InfoGraph` fields public.
* Move `graph_style` into `InfoGraph`.
* Rename `GraphStyle::Boxes` to `GraphStyle::Box`.
* Remove `InfoGraphBuilder`, and move builder methods onto `InfoGraph`.
* Add `InfoGraph.graphviz_attrs` to specify GraphViz specific attributes.
* Add `GraphvizAttrs.edge_constraint_default` to specify the default `constraint` for all edges.
* Add `GraphvizAttrs.edge_constraints` to specify the `constraint` for each edge.
* Add `GraphvizAttrs.edge_minlen_default` to specify the default `minlen` for all edges.
* Add `GraphvizAttrs.edge_minlens` to specify the `minlen` for each edge.
* Add `GraphvizAttrs.edge_dir_default` to specify the default `dir` for all edges.
* Add `GraphvizAttrs.edge_dirs` to specify the `dir` for each edge.
* Change order that nodes are provided to GraphViz for vertical graphs.


## 0.6.0 (2024-06-09)

* SVGs copied now use `&#160;` for spaces for valid SVG XML.
* Edges between clusters use the middle node instead of last + first nodes.
* Support adding labels to edges via `edge_descs` in `InfoGraph`.


## 0.5.0 (2024-03-30)

* In `InfoGraph`, replace `TailwindClasses` with inline `theme`.
* Support YAML merge keys in playground.


## 0.4.1 (2024-03-14)

* Include `dot_ix::model::common::` module prefix for `node_id!` and `edge_id!` macros.
* Re-export `IndexMap`.


## 0.4.0 (2024-03-14)

* Include `dot_ix::model::` module prefix for `node_id!` and `edge_id!` macros.
* Gate `dot_ix_rt` behind `"rt"` feature, enabled by default.
* Gate `dot_ix_web_components` behind `"web_components"` feature, enabled by default.


## 0.3.0 (2024-03-13)

* Reverse order of nodes fed to graphviz.
* Add `FlexDiag` web component behind `"flex_diag"` feature.


## 0.2.0 (2024-02-22)

* Split `dot_ix` library and `dot_ix_playground` binary.
* Add `InfoGraph::builder` and `InfoGraphBuilder`.
* Add `NodeId::new`.


## 0.1.0 (2024-02-04)

* Experimental release
* Ability to generate graphs in a web application: [demo](https://azriel.im/dot_ix/).
