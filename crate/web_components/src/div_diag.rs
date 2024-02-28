//! Diagram created using HTML `div` elements.

use dot_ix_model::{
    common::NodeHierarchy,
    info_graph::{InfoGraph, NodeInfo},
};
use leptos::*;

fn divs(info_graph: InfoGraph, hierarchy: NodeHierarchy) -> impl IntoView {
    view! {
        <For
            each=move || hierarchy.clone().into_inner().into_iter()
            key=|(node_id, _node_hierarchy)| node_id.clone()
            children=move |(node_id, child_hierarchy)| {
                let node_infos = info_graph.node_infos();
                let node_info = node_infos.get(&node_id);
                let emoji = node_info.and_then(NodeInfo::emoji).map(str::to_string).unwrap_or_default();
                let name = node_info.map(NodeInfo::name).map(str::to_string).unwrap_or_default();
                let desc = node_info.and_then(NodeInfo::desc).map(str::to_string).unwrap_or_default();
                view! {
                    <div>
                        {emoji} {name} {desc}
                        {divs(info_graph.clone(), child_hierarchy.clone())}
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
        {move || {
            let info_graph = info_graph.get();
            let root_nodes = info_graph.hierarchy();
            divs(info_graph.clone(), root_nodes.clone())}
        }
    }
}
