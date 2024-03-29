# Changelog

## unreleased

* In `InfoGraph`, replace `TailwindClasses` with inline `theme`.


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
