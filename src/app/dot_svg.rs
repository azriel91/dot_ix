use leptos::{html::Div, *};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/js/graphviz_dot_svg.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn graphviz_dot_svg(dot_src: String) -> Result<String, JsValue>;
}

/// Renders a graphviz graph as an SVG.
#[component]
pub fn DotSvg(cx: Scope, dot_src: ReadSignal<Option<String>>) -> impl IntoView {
    // DOM elements for the graph and error
    let svg_div_ref = create_node_ref::<Div>(cx);

    let (error_text, set_error_text) = create_signal(cx, None::<String>);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _dot_src = dot_src;
        let _set_error_text = set_error_text;
    }

    create_effect(cx, move |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(dot_src) = dot_src.get() {
            if !dot_src.is_empty() {
                use std::borrow::Cow;

                let (dot_svg, error) = match graphviz_dot_svg(dot_src) {
                    Ok(dot_svg) => (Cow::Owned(dot_svg), None),
                    Err(error) => {
                        let error = js_sys::Error::from(error)
                            .to_string()
                            .as_string()
                            .unwrap_or_else(|| String::from("<unknown>"));

                        (Cow::Borrowed(""), Some(error))
                    }
                };

                if let Some(svg_div) = svg_div_ref.get() {
                    svg_div.set_inner_html(&dot_svg);
                }

                // ⚠️ Normally we should not write to a signal in `create_effect`, as it causes
                // state to be out of sync between server and client.
                //
                // However, for a client-side only component, we don't need to keep in sync with
                // the server.
                set_error_text.update(|error_text| *error_text = error);
            }
        }
    });

    view! { cx,
        <Suspense
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            <h2>"Graph"</h2>
            <div>
                <div id="svg_div" _ref=svg_div_ref />
                <div
                    id="error_div"
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
        </Suspense>
    }
}
