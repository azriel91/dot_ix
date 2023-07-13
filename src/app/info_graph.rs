use leptos::*;

use crate::{app::DotSvg, model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};

/// Text input and dot graph rendering.
#[component]
pub fn InfoGraph(cx: Scope) -> impl IntoView {
    let (info_graph_src, set_info_graph_src) = create_signal(
        cx,
        String::from(
            r#"---
hierarchy:
  node_a:
    node_a0: {}
    node_a1: {}
  node_b:
    node_b0: {}

node_infos:
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
    let (error_text, set_error_text) = create_signal(cx, None::<String>);
    let (dot_src, set_dot_src) = create_signal(cx, None::<String>);
    let info_graph_parse = move |_| {
        let info_graph_result =
            serde_yaml::from_str::<crate::model::info_graph::InfoGraph>(&info_graph_src.get());
        let info_graph_result = &info_graph_result;

        set_dot_src.update(|dot_src| match info_graph_result {
            Ok(info_graph) => {
                *dot_src = Some(IntoGraphvizDotSrc::into(
                    info_graph,
                    &GraphvizDotTheme::default(),
                ))
            }
            Err(_) => {
                *dot_src = None;
            }
        });
        set_error_text.update(|error_text| match info_graph_result {
            Ok(_) => *error_text = None,
            Err(error) => *error_text = Some(format!("{error}")),
        });
    };

    view! { cx,
        <div
            class="grid grid-cols-3">

            <div>
                <label for="info_graph_yml">"info_graph.yml"</label><br/>
                <textarea
                    id="info_graph_yml"
                    name="info_graph_yml"
                    rows="40"
                    cols="80"
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
                <br />
                <div class={
                        let error_text = error_text.get();
                        let error_text_empty = error_text.as_deref().map(str::is_empty).unwrap_or(true);
                        move || {
                            if error_text_empty {
                                "hidden"
                            } else {
                                "
                                border
                                border-amber-300
                                bg-gradient-to-b from-amber-100 to-amber-200
                                rounded
                                "
                            }
                        }
                    }
                    >{error_text}</div>
                <br />
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
                <DotSvg dot_src=dot_src />
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
                    cols="80"
                    prop:value={dot_src} />
            </div>
        </div>
    }
}
