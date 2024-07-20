use std::time::Duration;

use dot_ix::{
    model::{
        common::{DotSrcAndStyles, GraphvizDotTheme},
        info_graph::InfoGraph,
    },
    rt::IntoGraphvizDotSrc,
    web_components::{DotSvg, FlexDiag},
};
use leptos::*;

use crate::app::{TabLabel, TextEditor};

#[cfg(target_arch = "wasm32")]
use super::QUERY_PARAM_DIAGRAM_ONLY;

#[cfg(target_arch = "wasm32")]
const INFO_GRAPH_DEMO: &str = include_str!("info_graph_example.yaml");

/// User provided info graph source.
#[cfg(target_arch = "wasm32")]
const QUERY_PARAM_SRC: &str = "src";

#[cfg(not(target_arch = "wasm32"))]
fn info_graph_src_init() -> String {
    String::from("")
}

/// Sets the info graph src using logic purely executed on the client side.
///
/// This is for a pure client side rendered app, so updating a signal within
/// `create_effect` is safe.
#[cfg(target_arch = "wasm32")]
fn info_graph_src_init() -> String {
    use js_sys::Array;
    use lz_str::decompress_from_encoded_uri_component;
    use web_sys::{console, Document, Url, UrlSearchParams};

    if let Some(window) = web_sys::window() {
        let url_search_params = {
            let url = Url::new(&String::from(window.location().to_string()))
                .expect("Expected URL to be valid.");

            let hash = url.hash();
            if hash.is_empty() {
                Some(url.search_params())
            } else {
                let hash = hash.replacen('#', "?", 1);
                match UrlSearchParams::new_with_str(hash.as_str()) {
                    Ok(search_params) => Some(search_params),
                    Err(error) => {
                        let message = Array::new_with_length(1);
                        message.set(0, error);
                        console::log(&message);
                        None
                    }
                }
            }
        };

        if let Some(url_search_params) = url_search_params {
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

            info_graph_src_initial
        } else {
            String::from(INFO_GRAPH_DEMO)
        }
    } else {
        String::from("# Could not extract search params.")
    }
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
    let (info_graph_src, set_info_graph_src) = create_signal(info_graph_src_init());
    let flex_diag_radio = create_node_ref::<html::Input>();
    let (flex_diag_visible, flex_diag_visible_set) = create_signal(false);
    let flex_diag_visible_update = move |_ev| {
        let flex_diag_checked = flex_diag_radio
            .get()
            .map(|input| input.checked())
            .unwrap_or(true);
        leptos::logging::log!("flex_diag_checked: {flex_diag_checked}");
        flex_diag_visible_set.set(flex_diag_checked);
    };

    let editor_and_disclaimer_wrapper_classes = "\
        flex \
        items-start \
        flex-nowrap \
        flex-col \
    ";
    let editor_and_diagram_div_classes = move || {
        if diagram_only.get() {
            "
            basis-auto \
            grow \
            \
            flex \
            flex-row \
            justify-center \
            "
        } else {
            "\
            basis-auto \
            grow \
            \
            flex \
            flex-row \
            items-start \
            mb-10 \
            flex-wrap \
            lg:flex-row \
            lg:flex-nowrap \
            "
        }
    };
    let textbox_div_display_classes = move || {
        if diagram_only.get() {
            "hidden"
        } else {
            "\
            basis-1/2 \
            grow \
            \
            mb-10 \
            lg:mb-0 \
            \
            lg:w-[40dvw] \
            h-[50dvh] \
            lg:h-[78dvh] \
            \
            lg:basis-2/5 \
            lg:grow-0 \
            "
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

    let (info_graph, set_info_graph) = create_signal(InfoGraph::default());

    create_effect(move |_| {
        let info_graph_src = info_graph_src.get();

        let merge_key_exists = info_graph_src.lines().any(|line| {
            // skip whitespace, but ignore comments
            line.trim_start().starts_with("<<:")
        });
        let info_graph_result = if merge_key_exists {
            let info_graph_value = serde_yaml::from_str::<serde_yaml::Value>(&info_graph_src);
            info_graph_value
                .and_then(|mut value| {
                    value.apply_merge()?;
                    Ok(value)
                })
                .and_then(serde_yaml::from_value::<InfoGraph>)
        } else {
            // Better diagnostics, numbers don't have to be quoted to be strings.
            serde_yaml::from_str::<InfoGraph>(&info_graph_src)
        };
        let info_graph_result = &info_graph_result;

        match info_graph_result {
            Ok(info_graph) => {
                set_info_graph.set(info_graph.clone());

                let DotSrcAndStyles { dot_src, styles } =
                    IntoGraphvizDotSrc::into(info_graph, &GraphvizDotTheme::default());

                set_dot_src.set(Some(dot_src));
                set_styles.set(Some(styles));
                set_error_text.set(None);
                #[cfg(target_arch = "wasm32")]
                {
                    use lz_str::compress_to_encoded_uri_component;

                    let src_compressed = compress_to_encoded_uri_component(&info_graph_src);
                    if let Some(window) = web_sys::window() {
                        let url = {
                            let url =
                                web_sys::Url::new(&String::from(window.location().to_string()))
                                    .expect("Expected URL to be valid.");

                            // Remove this in a few versions.
                            url.search_params().delete(QUERY_PARAM_SRC);
                            url.search_params().delete(QUERY_PARAM_DIAGRAM_ONLY);

                            let fragment = {
                                let mut fragment = String::with_capacity(
                                    QUERY_PARAM_SRC.len() + src_compressed.len() + 64,
                                );
                                fragment.push_str(QUERY_PARAM_SRC);
                                fragment.push_str("=");
                                fragment.push_str(&src_compressed);

                                if diagram_only.get() {
                                    fragment.push_str("&");
                                    fragment.push_str(QUERY_PARAM_DIAGRAM_ONLY);
                                    fragment.push_str("=true");
                                }

                                fragment
                            };
                            url.set_hash(&fragment);
                            url.to_string()
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
        <div class={ editor_and_disclaimer_wrapper_classes }>
            <div class={ editor_and_diagram_div_classes }>
                <div class={ textbox_div_display_classes }>

                    <TabLabel
                        tab_group_name="src_tabs"
                        tab_id="tab_info_graph_yml"
                        label="info_graph.yml"
                        checked=true
                    />

                    <TabLabel
                        tab_group_name="src_tabs"
                        tab_id="tab_info_graph_dot"
                        label="generated.dot"
                    />

                    // tab content
                    <div class="\
                        hidden \
                        peer-checked/tab_info_graph_yml:block \
                        \
                        h-full \
                        "
                    >
                        <TextEditor
                            value=info_graph_src
                            set_value=set_info_graph_src
                            id="info_graph_yml"
                            name="info_graph_yml"
                            class="\
                                border \
                                border-slate-400 \
                                bg-slate-100 \
                                font-mono \
                                h-full \
                                p-2 \
                                rounded \
                                text-xs \
                            "
                        />
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

                    // tab content
                    <div class="\
                        hidden \
                        peer-checked/tab_info_graph_dot:block \
                        \
                        h-full \
                        "
                    >
                        <textarea
                            id="info_graph_dot"
                            name="info_graph_dot"
                            class="\
                                border \
                                border-slate-400 \
                                bg-slate-100 \
                                w-full \
                                h-full \
                                font-mono \
                                p-2 \
                                rounded \
                                text-xs \
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
                <div
                    class={move || {
                        if diagram_only.get() {
                            ""
                        } else {
                            // Take up the full screen if the screen size is small,
                            // otherwise take up just enough for content.
                            "\
                            basis-1/2 \
                            grow \
                            lg:basis-3/5 \
                            lg:grow \
                            "
                        }
                    }}
                >

                    <TabLabel
                        tab_group_name="diagram_tabs"
                        tab_id="tab_dot_svg"
                        label="Dot SVG"
                        checked=true
                        on_change=flex_diag_visible_update
                    />

                    <TabLabel
                        tab_group_name="diagram_tabs"
                        tab_id="tab_flex_diag"
                        label="Flex Diagram"
                        on_change=flex_diag_visible_update
                        node_ref=flex_diag_radio
                    />

                    <div
                        class="\
                            hidden \
                            peer-checked/tab_dot_svg:block \
                        "
                    >
                        <DotSvg
                            dot_src_and_styles=dot_src_and_styles.into()
                            diagram_only=diagram_only.into()
                        />
                    </div>

                    <div
                        class="\
                            hidden \
                            peer-checked/tab_flex_diag:block \
                        "
                    >
                        <FlexDiag
                            info_graph=info_graph
                            visible=flex_diag_visible.into()
                        />
                    </div>
                </div>
            </div>
            <Disclaimer diagram_only />
        </div>
    }
}

#[component]
pub fn Disclaimer(diagram_only: ReadSignal<bool>) -> impl IntoView {
    let disclaimer_classes = move || {
        if diagram_only.get() {
            "hidden"
        } else {
            "
            basis-auto \
            \
            border \
            border-amber-300 \
            bg-gradient-to-b from-amber-100 to-amber-200 \
            inline-block \
            my-2 \
            p-2 \
            rounded \
            "
        }
    };

    view! {
        <div class=disclaimer_classes>
            <p>
                <span class="font-bold">"🐱 GitHub: "</span>
                <a
                    class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                    href="https://github.com/azriel91/dot_ix"
                >
                    "azriel91/dot_ix"
                </a>
            </p>
            <p>
                "This is an "<i>"early"</i>" frontend prototype for the "
                <a
                    class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                    href="https://peace.mk"
                >
                    "Peace"
                </a>
                " automation framework"
            </p>
            <p>
                <b>"Notes:"</b>
                <ol class="list-disc mx-4">
                <li>"URLs are shareable: the graph is stored in the bookmark fragment."</li>
                <li>"The Flex Diagram is still experimental, and the arrows don't \
                    receive styling."</li>
                <li>"Docs for GraphViz attributes can be found at: "<a
                    class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                    href="https://docs.rs/dot_ix_model/latest/dot_ix_model/common/graphviz_attrs/struct.GraphvizAttrs.html"
                >
                    "docs.rs: GraphvizAttrs"
                </a></li>
                <li>"Docs for theme keys can be found at: "<a
                    class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                    href="https://docs.rs/dot_ix_model/latest/dot_ix_model/theme/enum.ThemeAttr.html"
                >
                    "docs.rs: ThemeAttr"
                </a></li>
                <li>"If you'd like to contribute to the development of this library, \
                    I'd be super delighted, since I'm not a web-dev™️ 🙃."</li>
                </ol>
            </p>
        </div>
    }
}
