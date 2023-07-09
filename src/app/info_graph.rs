use leptos::{
    component, create_signal, event_target_value, tracing, view, IntoView, Scope, SignalGet,
    SignalUpdate,
};

use crate::{model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};

/// Renders the home page of your application.
#[component]
pub fn InfoGraph(cx: Scope) -> impl IntoView {
    let (info_graph_src, set_info_graph_src) = create_signal(
        cx,
        String::from(
            r#"---
nodes:
  node_a: !Info
    name: "⚙️ Node A"
    desc: Contains things to do with A.
  node_a0: !Name "A Child 0" # shorthand
  node_a1: !Name "A Child 1"
  node_b : !Name "Node B"
  node_b0: !Name "B Child 0"

edges:
  edge_a_b: [node_a, node_b]
  edge_a1_b0: [node_a1, node_b0]

children:
  node_a: [node_a0, node_a1]
  node_b: [node_b0]

node_tags:
  node_a: [tag_0, tag_1]
  node_a0: [tag_0]
  node_a1: [tag_1]
  node_b: [tag_0]
  node_b0: [tag_0]

# tags are not necessarily associated with a node.
tags:
  tag_0: !Info { name: "Tag 0", desc: "Some information for tag 0." }
  tag_1: !Name "Tag 1"
  tag_2: !Name "Tag 2"
"#,
        ),
    );
    // Creates a reactive value to update the button
    let (dot_src, set_dot_src) = create_signal(cx, String::from(""));
    let info_graph_parse = move |_| {
        set_dot_src.update(|dot_src| {
            match serde_yaml::from_str::<crate::model::InfoGraph>(&info_graph_src.get()) {
                Ok(info_graph) => {
                    *dot_src = IntoGraphvizDotSrc::into(&info_graph, &GraphvizDotTheme::default())
                }
                Err(error) => *dot_src = format!("{error}"),
            }
        });
    };

    view! { cx,
        <div
            class="grid grid-cols-2">

            <div>
                <label for="info_graph_yml">"info_graph.yml"</label><br/>
                <textarea
                    id="info_graph_yml"
                    name="info_graph_yml"
                    rows="40"
                    cols="100"
                    class="
                        border
                        border-slate-400
                        bg-slate-100
                        font-mono
                        p-2
                        rounded
                        text-xs
                    "
                    on:input=move |ev| { set_info_graph_src(event_target_value(&ev)) }
                    prop:value=info_graph_src />
                <br/>
                <button
                    class="
                        border-2
                        border-slate-500
                        bg-gradient-to-b from-slate-400 to-slate-500
                        font-bold
                        px-3.5
                        py-0.5
                        rounded-xl
                        text-white

                        hover:border-slate-400
                        hover:from-slate-300 hover:to-slate-400
                        hover:border-slate-500
                        hover:bg-slate-400
                        hover:text-white
                    "
                    on:click=info_graph_parse>"Parse"</button>
            </div>
            <div>
                <label for="info_graph_dot">"info_graph.dot"</label><br/>
                <textarea
                    id="info_graph_dot"
                    name="info_graph_dot"
                    class="
                        border
                        border-slate-400
                        bg-slate-100
                        font-mono
                        p-2
                        rounded
                        text-xs
                    "
                    rows="40"
                    cols="100"
                    prop:value=dot_src />
            </div>
        </div>
    }
}
