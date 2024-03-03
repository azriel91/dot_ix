//! Diagram created using HTML `div` elements.
//!
//! Lines between `div`s are drawn using [`leader-line`].
//!
//! The `leader-line.min.js` is retrieved using the command:
//!
//! ```bash
//! curl -Lfs --remote-name https://cdn.jsdelivr.net/npm/leader-line/leader-line.min.js
//! ```
//!
//! [`leader-line`]: https://anseki.github.io/leader-line/

use std::rc::Rc;

use dot_ix_model::{
    common::{NodeHierarchy, NodeId},
    info_graph::{IndexMap, InfoGraph, NodeInfo},
};
use leptos::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type LeaderLine;

    #[wasm_bindgen(method)]
    fn remove(this: &LeaderLine);
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DashOpts {
    pub animation: bool,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LeaderLineOpts {
    pub color: String,
    pub dash: DashOpts,
    pub size: u32,
    pub startSocketGravity: u32,
    pub endSocketGravity: u32,
    pub endPlugSize: f64,
    pub classes: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/js/leader-line.min.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn leader_line(
        src_id: &str,
        dest_id: &str,
        opts: &JsValue,
    ) -> Result<Option<LeaderLine>, JsValue>;
}

const NODE_CLASSES: &str = "\
    node \
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
const NODE_LABEL_WRAPPER_CLASSES: &str = "\
    flex \
    flex-initial \
    justify-center \
    gap-x-4 \
";
const NODE_EMOJI_WRAPPER_CLASSES: &str = "\
    flex \
    flex-col \
    justify-center \
";
const NODE_NAME_DESC_WRAPPER_CLASSES: &str = "\
    flex \
    flex-col \
    content-start \
";
const NODE_CHILDREN_WRAPPER_CLASSES: &str = "\
    flex \
    flex-initial \
    justify-center \
    gap-4 \
";
const NODE_CHILDREN_VERT_WRAPPER_CLASSES: &str = "\
    flex \
    flex-col \
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

                let node_classes = info_graph.tailwind_classes()
                    .node_classes(node_id.clone())
                    .unwrap_or(NODE_CLASSES)
                    .to_string();

                // Partition children from this node's child hierarchy, based on their rank.
                let child_hierarchy_groups = child_hierarchy.into_inner().into_iter()
                    .fold(IndexMap::<NodeId, NodeHierarchy>::new(), |mut groups, (node_id, sub_hierarchy)| {
                        // key: the last node id in the hierarchy
                        // val: the group to put the current node into

                        let predecessor_node_id = info_graph.edges().iter().find_map(|(_edge_id, [src, dest])| {
                            if dest == &node_id {
                                Some(src)
                            } else {
                                None
                            }
                        });

                        if let Some(mut node_hierarchy) = predecessor_node_id
                            .and_then(|predecessor_node_id| groups.shift_remove(predecessor_node_id))
                        {
                            node_hierarchy.insert(node_id.clone(), sub_hierarchy);
                            groups.insert(node_id, node_hierarchy);
                        } else {
                            let mut node_hierarchy = NodeHierarchy::new();
                            node_hierarchy.insert(node_id.clone(), sub_hierarchy);
                            groups.insert(node_id, node_hierarchy);
                        }

                        groups
                    });

                let info_graph = Rc::clone(&info_graph);

                view! {
                    <div id={move || node_id.to_string()} tabindex="0" class=node_classes>
                        <div class=NODE_LABEL_WRAPPER_CLASSES>
                            <div class=NODE_EMOJI_WRAPPER_CLASSES><div>{emoji}</div></div>
                            <div class=NODE_NAME_DESC_WRAPPER_CLASSES>
                                <div>{name}</div>
                                <div>{desc}</div>
                            </div>
                        </div>
                        <div class=NODE_CHILDREN_WRAPPER_CLASSES>
                            <For
                                each=move || child_hierarchy_groups.clone().into_iter()
                                key=|(node_id, _node_hierarchy)| format!("{node_id}_group")
                                children=move |(_node_id, child_hierarchy_group)| {
                                    view! {
                                        <div class=NODE_CHILDREN_VERT_WRAPPER_CLASSES>
                                            {divs(Rc::clone(&info_graph), child_hierarchy_group)}
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                }
            }
        />
    }
}

/// Renders a diagram using `div`s.
#[component]
pub fn DivDiag(info_graph: ReadSignal<InfoGraph>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    let (leader_lines, leader_lines_set) = create_signal(Vec::<LeaderLine>::new());

    view! {
        <div class="flex flex-initial items-stretch">
        { move || {
            let info_graph = Rc::new(info_graph.get());
            let root_nodes = info_graph.hierarchy().clone();
            divs(info_graph, root_nodes)
        } }
        { move || {
            #[cfg(not(target_arch = "wasm32"))]
            let _info_graph = info_graph.get();

            #[cfg(target_arch = "wasm32")]
            {
                let leader_lines = leader_lines.get();
                leader_lines.iter().for_each(LeaderLine::remove);

                let info_graph = info_graph.get();
                let edges = info_graph.edges();
                let tailwind_classes = info_graph.tailwind_classes();

                let leader_lines = edges
                    .iter()
                    .fold(
                        Vec::with_capacity(edges.len()),
                        |mut leader_lines, (edge_id, [src, dest])|
                        {
                            let classes = tailwind_classes
                                .edge_classes_or_default(edge_id.clone())
                                .to_string();

                            let opts = LeaderLineOpts {
                                color: "#336699".to_string(),
                                dash: DashOpts {
                                    animation: true,
                                },
                                size: 3,
                                startSocketGravity: 20,
                                endSocketGravity: 40,
                                endPlugSize: 1.2,
                                classes,
                            };
                            if let Ok(Some(leader_line)) = leader_line(
                                src,
                                dest,
                                &serde_wasm_bindgen::to_value(&opts).unwrap(),
                            ) {
                                leader_lines.push(leader_line);
                            }

                            leader_lines
                        });

                leader_lines_set.set(leader_lines);
            }
        } }
        </div>
    }
}
