use std::time::Duration;

use leptos::*;

use crate::{
    app::DotSvg,
    model::common::{DotSrcAndStyles, GraphvizDotTheme},
    rt::IntoGraphvizDotSrc,
};

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
    use lz_str::decompress_from_encoded_uri_component;
    use web_sys::{Document, Url};

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let url_search_params = Url::new(&String::from(window.location().to_string()))
                .expect("Expected URL to be valid.")
                .search_params();
            let info_graph_src_initial = url_search_params
                .get(QUERY_PARAM_SRC)
                .map(|src| {
                    if src.contains("\n") {
                        // Treat src as plain yaml
                        src
                    } else {
                        // Try deserialize/serialize src as lz_str
                        decompress_from_encoded_uri_component(&src).map_or_else(
                            || format!("# deserialize src error: invalid data"),
                            |s| {
                                String::from_utf16(&s).unwrap_or_else(|_| {
                                    format!("# deserialize src error: invalid data")
                                })
                            },
                        )
                    }
                })
                .unwrap_or_else(|| String::from(INFO_GRAPH_DEMO));

            set_info_graph_src.set(info_graph_src_initial);

            // Hack: Get Tailwind CSS from CDN to reevaluate document.
            set_timeout(
                move || {
                    let _ = window
                        .document()
                        .as_ref()
                        .and_then(Document::body)
                        .as_deref()
                        .map(|element| element.append_with_str_1(""));
                },
                Duration::from_millis(200),
            );
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

    let layout_classes = move || {
        if diagram_only.get() {
            "flex items-start"
        } else {
            "flex items-start flex-wrap"
        }
    };
    let textbox_display_classes = move || {
        if diagram_only.get() {
            "hidden"
        } else {
            "tabs basis-1/2 grow"
        }
    };

    // Creates a reactive value to update the button
    let (error_text, set_error_text) = create_signal(None::<String>);
    let (dot_src, set_dot_src) = create_signal(None::<String>);
    let (styles, set_styles) = create_signal(None::<String>);
    let dot_src_and_styles = move || {
        dot_src
            .get()
            .zip(styles.get())
            .map(|(dot_src, styles)| DotSrcAndStyles { dot_src, styles })
    };

    #[cfg(target_arch = "wasm32")]
    info_graph_src_init(set_info_graph_src);

    create_effect(move |_| {
        let info_graph_result =
            serde_yaml::from_str::<crate::model::info_graph::InfoGraph>(&info_graph_src.get());
        let info_graph_result = &info_graph_result;

        match info_graph_result {
            Ok(info_graph) => {
                let DotSrcAndStyles { dot_src, styles } =
                    IntoGraphvizDotSrc::into(info_graph, &GraphvizDotTheme::default());

                set_dot_src.set(Some(dot_src));
                set_styles.set(Some(styles));
                set_error_text.set(None);
                #[cfg(target_arch = "wasm32")]
                {
                    use lz_str::compress_to_encoded_uri_component;

                    let src_compressed = compress_to_encoded_uri_component(&info_graph_src.get());
                    if let Some(window) = web_sys::window() {
                        let url = {
                            let u = web_sys::Url::new(&String::from(window.location().to_string()))
                                .expect("Expected URL to be valid.");
                            u.search_params().set(QUERY_PARAM_SRC, &src_compressed);
                            u.to_string()
                                .as_string()
                                .expect("# Failed to decode src parameter")
                        };

                        let _ = window
                            .history() // should this panic if url cannot be modified?
                            .and_then(|h| {
                                h.replace_state_with_url(&"".into(), "".into(), Some(&url))
                            });
                    }
                }
            }
            Err(error) => {
                set_dot_src.set(None);
                set_styles.set(None);
                set_error_text.set(Some(format!("{error}")));
            }
        }
    });

    view! {
        <div class={ move || layout_classes() }>
            <div class={ move || textbox_display_classes() }>

                <input type="radio" name="tabs" id="tab_info_graph_yml" checked="checked" />
                <label for="tab_info_graph_yml">"info_graph.yml"</label>

                <div class="tab">
                    <textarea
                        id="info_graph_yml"
                        name="info_graph_yml"
                        class="
                            border
                            border-slate-400
                            bg-slate-100
                            font-mono
                            min-w-full
                            min-h-full
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

                <input type="radio" name="tabs" id="tab_info_graph_dot" />
                <label for="tab_info_graph_dot">"info_graph.dot"</label>
                <div class="tab">
                    <textarea
                        id="info_graph_dot"
                        name="info_graph_dot"
                        class="
                            border
                            border-slate-400
                            bg-slate-100
                            min-w-full
                            min-h-full
                            font-mono
                            p-2
                            rounded
                            text-xs
                        "
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
            <div class="diagram basis-1/2 grow">
                <DotSvg
                    dot_src_and_styles=dot_src_and_styles
                    diagram_only=diagram_only
                />
            </div>
        </div>
    }
}
