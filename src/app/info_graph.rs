use std::time::Duration;

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
  a:
    a0:
    a1:
  b:
    b0:
  c:
  d:

node_infos:
  a:
    name: "⚙️ Node A"
    desc: >
      Contains things to
      do with A.

      More description
  a0: { name: "A0", desc: "something to do with A0" }
  a1: { name: "A1" }
  b : { name: "B" }
  b0: { name: "B0" }
  c:  { name: "C" }
  d:  { name: "D" }

edges:
  ab: [a, b]
  a0a1: [a0, a1]
  a0b0: [a0, b0]
  bc: [b, c]
  bd: [b, d]

node_tags:
  a: [tag_0, tag_1]
  a0: [tag_0]
  a1: [tag_1]
  b: [tag_0]
  b0: [tag_0]

# tags are not necessarily associated with a node.
tags:
  tag_0: { name: "Tag 0", desc: "Some information for tag 0." }
  tag_1: { name: "Tag 1" }
  tag_2: { name: "Tag 2" }
"#,
        ),
    );
    // Creates a reactive value to update the button
    let (error_text, set_error_text) = create_signal(cx, None::<String>);
    let (dot_src, set_dot_src) = create_signal(cx, None::<String>);
    create_effect(cx, move |_| {
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
    });

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
                    on:input=leptos_dom::helpers::debounce(cx, Duration::from_millis(400), move |ev| {
                        let info_graph_src = event_target_value(&ev);
                        set_info_graph_src(info_graph_src);
                    })

                    prop:value=info_graph_src />
                <br />
                <div
                    class={
                        move || {
                            let error_text = error_text.get();
                            let error_text_empty = error_text
                                .as_deref()
                                .map(str::is_empty)
                                .unwrap_or(true);
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
                    >{
                        move || {
                            let error_text = error_text.get();
                            error_text.as_deref()
                                .unwrap_or("")
                                .to_string()
                        }
                    }</div>
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
                    on:input=leptos_dom::helpers::debounce(cx, Duration::from_millis(400), move |ev| {
                        let dot_src = event_target_value(&ev);
                        set_dot_src(Some(dot_src));
                    })
                    prop:value={
                        move || {
                            let dot_src = dot_src.get();
                            dot_src.as_deref()
                                .unwrap_or("")
                                .to_string()
                        }
                    } />
            </div>
        </div>
    }
}
