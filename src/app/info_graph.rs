use std::time::Duration;

use leptos::*;

use crate::{app::DotSvg, model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};

const INFO_GRAPH_DEMO: &str = include_str!("info_graph_example.yaml");

/// User provided info graph source.
#[cfg(target_arch = "wasm32")]
const QUERY_PARAM_SRC: &str = "src";

/// Sets the info graph src using logic purely executed on the client side.
///
/// This is for a pure client side rendered app, so updating a signal withing
/// `create_effect` is safe.
#[cfg(target_arch = "wasm32")]
fn info_graph_src_init(set_info_graph_src: WriteSignal<String>) {
    use web_sys::Url;

    create_effect(move |_| {
        if let Some(url_search_params) = web_sys::window().map(|window| {
            Url::new(&String::from(window.location().to_string()))
                .expect("Expected URL to be valid.")
                .search_params()
        }) {
            let info_graph_src_initial = url_search_params
                .get(QUERY_PARAM_SRC)
                .as_ref()
                .map(|src| {
                    serde_yaml::from_str::<crate::model::info_graph::InfoGraph>(src)
                        .map(|info_graph| {
                            serde_yaml::to_string(&info_graph)
                                .unwrap_or_else(|e| format!("# serialize src error: {e}"))
                        })
                        .unwrap_or_else(|e| format!("# deserialize src error: {e}"))
                })
                .unwrap_or_else(|| String::from(INFO_GRAPH_DEMO));

            set_info_graph_src.set(info_graph_src_initial);
        } else {
            set_info_graph_src.set(String::from("# Could not extract search params."));
        }
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
pub fn InfoGraph(diagram_only: ReadSignal<bool>) -> impl IntoView {
    let (info_graph_src, set_info_graph_src) = create_signal(String::from(INFO_GRAPH_DEMO));

    #[cfg(target_arch = "wasm32")]
    info_graph_src_init(set_info_graph_src);

    let layout_classes = move || {
        if diagram_only.get() {
            "grid grid-cols-1"
        } else {
            "grid grid-cols-3"
        }
    };
    let textbox_display_classes = move || {
        if diagram_only.get() { "hidden" } else { "" }
    };

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
        <div class={ move || layout_classes() }>
            <div class={ move || textbox_display_classes() }>
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
                    on:input=leptos_dom::helpers::debounce(Duration::from_millis(400), move |ev| {
                        let info_graph_src = event_target_value(&ev);
                        set_info_graph_src.set(info_graph_src);
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

            <div class={ move || textbox_display_classes() }>
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
