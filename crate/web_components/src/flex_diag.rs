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
    common::{AnyId, NodeHierarchy, NodeId},
    info_graph::{IndexMap, InfoGraph},
    theme::ElCssClasses,
};
use dot_ix_rt::InfoGraphHtml;
use leptos::{component, view, For, IntoView, Signal, SignalGet};

cfg_if::cfg_if! { if #[cfg(target_arch = "wasm32")] {
    use leptos::SignalSet;

    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen]
    extern "C" {
        #[derive(Clone)]
        pub type LeaderLine;

        #[wasm_bindgen(method)]
        fn remove(this: &LeaderLine);

        #[wasm_bindgen(method)]
        fn position(this: &LeaderLine);
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
    pub struct DashOpts {
        pub animation: bool,
    }

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

    #[wasm_bindgen(module = "/public/js/leader-line.min.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        fn leader_line(
            src_id: &str,
            dest_id: &str,
            opts: &JsValue,
        ) -> Result<Option<LeaderLine>, JsValue>;
    }
}}

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

/// Renders a diagram using `div`s.
#[component]
pub fn FlexDiag(
    #[prop(into)] info_graph: Signal<InfoGraph>,
    #[prop(default = Signal::from(|| true))] visible: Signal<bool>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    let (leader_lines, leader_lines_set) = leptos::create_signal(Vec::<LeaderLine>::new());

    let flex_diag_divs = move || {
        let info_graph = info_graph.get();

        let el_css_classes = el_css_classes(&info_graph);
        let root_nodes = info_graph.hierarchy().clone();

        let flex_divs_ctx = FlexDivsCtx {
            info_graph,
            el_css_classes,
        };
        let flex_divs_ctx = Rc::new(flex_divs_ctx);
        divs(flex_divs_ctx, root_nodes)
    };

    #[cfg(not(target_arch = "wasm32"))]
    let leader_lines_redraw = move || {
        let _info_graph = info_graph.get();
        let _visible = visible.get();
    };
    #[cfg(target_arch = "wasm32")]
    let leader_lines_redraw = move || {
        let leader_lines_vec = leader_lines.get();
        let visible = visible.get();

        if visible {
            let info_graph = info_graph.get();
            let edges = info_graph.edges();
            let el_css_classes = el_css_classes(&info_graph);

            let leader_lines_new = edges.iter().fold(
                Vec::with_capacity(edges.len()),
                |mut leader_lines_new, (edge_id, [src, dest])| {
                    let classes = el_css_classes
                        .get(&AnyId::from(edge_id.clone()))
                        .map(AsRef::<str>::as_ref)
                        .unwrap_or_default()
                        .to_string();

                    let opts = LeaderLineOpts {
                        color: "#336699".to_string(),
                        dash: DashOpts { animation: true },
                        size: 3,
                        startSocketGravity: 20,
                        endSocketGravity: 40,
                        endPlugSize: 1.2,
                        classes,
                    };
                    if let Ok(Some(leader_line)) =
                        leader_line(src, dest, &serde_wasm_bindgen::to_value(&opts).unwrap())
                    {
                        leader_lines_new.push(leader_line);
                    }

                    leader_lines_new
                },
            );

            leader_lines_set.set(leader_lines_new.clone());

            // Hack to get leader-line render arrows after `div`s have been laid out.
            leptos::request_animation_frame(move || {
                leader_lines_new.iter().for_each(LeaderLine::position);
            });
        } else {
            leader_lines_set.set(Vec::new());
        }

        // Hack to remove leader-line arrows;
        leptos::request_animation_frame(move || {
            leader_lines_vec
                .into_iter()
                .for_each(|leader_line| leader_line.remove());
        });
    };

    view! {
        <div class="flex flex-initial items-stretch">
            { flex_diag_divs }
            { leader_lines_redraw }
        </div>
    }
}

fn el_css_classes(info_graph: &InfoGraph) -> ElCssClasses {
    let node_id_to_hierarchy = info_graph.hierarchy_flat();
    let info_graph_html = InfoGraphHtml {
        node_ids: node_id_to_hierarchy.keys().copied().collect::<Vec<_>>(),
        edge_ids: info_graph.edges().keys().collect::<Vec<_>>(),
    };
    info_graph.theme().el_css_classes(&info_graph_html)
}

fn divs(flex_divs_ctx: Rc<FlexDivsCtx>, hierarchy: NodeHierarchy) -> impl IntoView {
    view! {
        <For
            each=move || hierarchy.clone().into_inner().into_iter()
            key=|(node_id, _node_hierarchy)| node_id.clone()
            children=move |(node_id, child_hierarchy)| {
                let info_graph = &flex_divs_ctx.info_graph;
                let el_css_classes = &flex_divs_ctx.el_css_classes;

                let node_names = info_graph.node_names();
                let node_descs = info_graph.node_descs();
                let node_emojis = info_graph.node_emojis();
                let name = node_names.get(&node_id).cloned().unwrap_or_else(|| node_id.to_string());
                let desc = node_descs.get(&node_id).cloned().unwrap_or_default();
                let emoji = node_emojis.get(&node_id).cloned().unwrap_or_default();

                let node_classes = el_css_classes
                    .get(&AnyId::from(node_id.clone()))
                    .map(AsRef::<str>::as_ref)
                    .unwrap_or_default()
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

                let flex_divs_ctx = Rc::clone(&flex_divs_ctx);

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
                                            {divs(Rc::clone(&flex_divs_ctx), child_hierarchy_group)}
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

#[derive(Clone, Debug)]
struct FlexDivsCtx {
    info_graph: InfoGraph,
    el_css_classes: ElCssClasses,
}
