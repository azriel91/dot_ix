//! Diagram created using HTML `div` elements.

use std::rc::Rc;

use dot_ix_model::{
    common::NodeHierarchy,
    info_graph::{InfoGraph, NodeInfo},
};
use leptos::*;

const NODE_CLASSES: &str = "\
    node
    bg-slate-300 \
    border-2 \
    border-solid \
    p-2 \
    border-slate-600 \
    rounded-lg \
    focus:bg-lime-200 \
    focus:outline-1 \
    focus:outline-lime-600 \
    focus:outline-dashed \
    focus:rounded-lg \
    hover:[&:not(:has(.node:hover))]:bg-slate-200 \
    hover:[&:not(:has(.node:hover))]:border-slate-400 \
    focus:hover:[&:not(:has(.node:hover))]:bg-lime-200 \
    focus:hover:[&:not(:has(.node:hover))]:border-lime-400 \
    cursor-pointer \
";

fn divs(info_graph: Rc<InfoGraph>, hierarchy: NodeHierarchy) -> impl IntoView {
    view! {
        <For
            each=move || hierarchy.clone().into_inner().into_iter()
            key=|(node_id, _node_hierarchy)| node_id.clone()
            children=move |(node_id, child_hierarchy)| {
                let node_infos = info_graph.node_infos();
                let node_info = node_infos.get(&node_id);
                let emoji = node_info.and_then(NodeInfo::emoji).map(str::to_string).unwrap_or_default();
                let name = node_info.map(NodeInfo::name).map(str::to_string).unwrap_or_else(|| node_id.to_string());
                let desc = node_info.and_then(NodeInfo::desc).map(str::to_string).unwrap_or_default();
                view! {
                    <div id={move || node_id.to_string()} tabindex="0" class=NODE_CLASSES>
                        {emoji} {name} {desc}
                        {divs(Rc::clone(&info_graph), child_hierarchy)}
                    </div>
                }
            }
        />
    }
}

/// Renders a diagram using `div`s.
#[component]
pub fn DivDiag(info_graph: ReadSignal<InfoGraph>) -> impl IntoView {
    view! {
        <div class="flex flex-initial items-stretch">
        { move || {
            let info_graph = Rc::new(info_graph.get());
            let root_nodes = info_graph.hierarchy().clone();
            divs(info_graph, root_nodes)
        } }
        </div>
    }
}
