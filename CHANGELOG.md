# Changelog

## unreleased

* Add `TextEditor` in playground which uses [monaco][monaco] / [rust-monaco][rust-monaco].
* Add `GraphvizAttrs.pack_mode` to specify the `packmode` for subgraphs.

[monaco]: https://github.com/microsoft/monaco-editor
[rust-monaco]: https://github.com/siku2/rust-monaco


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
