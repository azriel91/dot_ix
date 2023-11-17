use std::time::Duration;

use leptos::*;

use crate::{app::DotSvg, model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};

const INFO_GRAPH_DEMO: &str = r#"---
hierarchy:
  a:
    a0:
      a00:
    a1:
  b:
    b0:
  c:
  d:

node_infos:
  a:
    emoji: 🛠️
    name: "Node A"
    desc: >
      Contains things to
      do with A.

      More description
  a0: { emoji: 🔨, name: "A0", desc: "something to do with A0" }
  a1: { emoji: ⚙️, name: "A1" }
  b : { name: "B" }
  b0: { name: "B0" }
  c:  { name: "C" }
  d:  { name: "D" }

edges:
  ab: [a, b]
  a0a1: [a00, a1]
  a0b0: [a0, b0]
  bc: [b, c]
  bd: [b, d]

node_tags:
  a: [tag_0]
  a0: [tag_1]
  a00: [tag_2]
  a1: [tag_1]
  b: [tag_0]
  b0: [tag_1]
  c: [tag_1, tag_2]
  d: [tag_2]

# tags are not necessarily associated with a node.
tags:
  tag_0: { name: "Tag 0", desc: "Some information for tag 0." }
  tag_1: { name: "Tag 1" }
  tag_2: { name: "Tag 2" }

tailwind_classes:
  tag_2:
    - '[&>path]:fill-blue-200'
    - '[&>path]:stroke-blue-500'
    - '[&>path]:hover:fill-blue-100'
    - '[&>path]:hover:stroke-blue-400'
    - '[&>path]:focus:fill-lime-200'
    - '[&>path]:focus:outline-1'
    - '[&>path]:focus:outline-lime-600'
    - '[&>path]:focus:outline-dashed'
    - '[&>path]:focus:rounded-xl'
"#;

#[cfg(feature = "server_side_graphviz")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server_side_graphviz")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoGraphQueryParams {
    #[serde(default)]
    src: Option<String>,
}

#[cfg(feature = "server_side_graphviz")]
#[server]
pub async fn info_graph_src_init() -> Result<InfoGraphQueryParams, ServerFnError> {
    use axum::extract::Query;
    use leptos_axum::*;

    let Query(info_graph_query_params) = extractor().await?;

    Ok(info_graph_query_params)
}

/// Query parameter name for the info graph source.
#[cfg(not(feature = "server_side_graphviz"))]
#[cfg(target_arch = "wasm32")]
const QUERY_PARAM_SRC: &str = "src";

/// Sets the info graph src using logic purely executed on the client side.
///
/// This is for a pure client side rendered app, so updating a signal withing
/// `create_effect` is safe.
#[cfg(not(feature = "server_side_graphviz"))]
#[cfg(target_arch = "wasm32")]
fn info_graph_src_init(set_info_graph_src: WriteSignal<String>) {
    use web_sys::{Url, UrlSearchParams};

    create_effect(move |_| {
        let info_graph_src_initial = web_sys::window()
            .and_then(|window| {
                let url_search_params: UrlSearchParams =
                    Url::new(&String::from(window.location().to_string()))
                        .expect("Expected URL to be valid.")
                        .search_params();

                url_search_params.get(QUERY_PARAM_SRC).as_ref().map(|src| {
                    serde_yaml::from_str::<crate::model::info_graph::InfoGraph>(src)
                        .map(|info_graph| {
                            serde_yaml::to_string(&info_graph)
                                .unwrap_or_else(|e| format!("# serialize src error: {e}"))
                        })
                        .unwrap_or_else(|e| format!("# deserialize src error: {e}"))
                })
            })
            .unwrap_or_else(|| String::from(INFO_GRAPH_DEMO));

        set_info_graph_src.update(|info_graph_src| *info_graph_src = info_graph_src_initial);
    });
}

/// Text input and dot graph rendering.
///
/// Notably, we run `set_*.update(..)` within a number of `create_effect`s.
///
/// While this is normally incorrect usage, for a purely client side
/// application, it is okay. From Greg (author of leptos):
///
/// > `create_effect` is also good for "only run this in the browser" and also
/// > for "synchronize with something non-reactive" (like a JS function) so
/// > don't worry about setting a signal inside it in that context.
/// >
/// > "Don't set a signal from an effect; rather, derive a signal." is advice
/// > meant in the sense "don't reactively read a signal inside an effect, and
/// > use it to set another signal". It's not the end of the world to do so,
/// > just not the best practice and can be hard to do correctly
#[component]
pub fn InfoGraph() -> impl IntoView {
    let (info_graph_src, set_info_graph_src) = create_signal(String::from(""));

    #[cfg(feature = "server_side_graphviz")]
    let src_init_resource = create_resource(|| (), |()| info_graph_src_init());

    #[cfg(not(feature = "server_side_graphviz"))]
    #[cfg(target_arch = "wasm32")]
    info_graph_src_init(set_info_graph_src);

    // Creates a reactive value to update the button
    let (error_text, set_error_text) = create_signal(None::<String>);
    let (dot_src, set_dot_src) = create_signal(None::<String>);
    create_effect(move |_| {
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

    view! {
        <div
            class="grid grid-cols-3">

            <div>
                <label for="info_graph_yml">"info_graph.yml"</label><br/>

                <Suspense fallback=move || view! {<p>"Loading src"</p> }>
                    {
                        move || {
                            #[cfg(feature = "server_side_graphviz")]
                            {
                                let info_graph_src_init = src_init_resource
                                    .get()
                                    .map(|info_graph_query_params_result| {
                                        info_graph_query_params_result
                                            .map(|info_graph_query_params| {
                                                leptos::logging::log!("successfully parsed info graph json");
                                                info_graph_query_params
                                                    .src
                                                    .as_deref()
                                                    .map(|src| {
                                                        serde_yaml::from_str::<crate::model::info_graph::InfoGraph>(src)
                                                            .map(|info_graph| {
                                                                serde_yaml::to_string(&info_graph)
                                                                    .unwrap_or_else(|e| format!("# serialize src error: {e}"))
                                                            }).unwrap_or_else(|e| format!("# deserialize src error: {e}"))

                                                    })
                                                    .unwrap_or_else(|| String::from("# src was nothing"))
                                            })
                                            .unwrap_or_else(|e| format!("# query params parse error: {e}"))
                                    })
                                    .unwrap_or_else(|| String::from(INFO_GRAPH_DEMO));
                                set_info_graph_src.set(info_graph_src_init);
                            }

                            view! {
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
                                    on:input=leptos_dom::helpers::debounce(Duration::from_millis(400), move |ev| {
                                        let info_graph_src = event_target_value(&ev);
                                        set_info_graph_src.set(info_graph_src);
                                    })

                                    prop:value=info_graph_src />
                            }
                        }
                    }
                </Suspense>

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
                    on:input=leptos_dom::helpers::debounce(Duration::from_millis(400), move |ev| {
                        let dot_src = event_target_value(&ev);
                        set_dot_src.set(Some(dot_src));
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
