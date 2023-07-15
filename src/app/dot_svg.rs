use leptos::{html::Div, *};
use leptos_meta::Script;

/// Renders a graphviz graph as an SVG.
#[component]
pub fn DotSvg(cx: Scope, dot_src: ReadSignal<Option<String>>) -> impl IntoView {
    // DOM elements for the graph and error
    let svg_div_ref = create_node_ref::<Div>(cx);
    let error_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        if let Some(dot_src) = dot_src.get() {
            if !dot_src.is_empty() {
                if svg_div_ref.get().is_some() {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(window) = web_sys::window() {
                        let _ = window.set_timeout_with_callback(&js_sys::Function::new_no_args(
                            &format!(
                                r#"
                                var dot_src = `{dot_src}`;
                                try {{
                                    var dot_svg = window.graphviz.layout(dot_src, "svg", "dot");
                                    document.getElementById("svg_div").innerHTML = dot_svg;
                                    document.getElementById("error_div").innerText = "";
                                }} catch (error) {{
                                    document.getElementById("error_div").innerText = error;
                                }}
                            "#
                            ),
                        ));
                    }
                }
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
                <div id="error_div" _ref=error_div_ref />

                // For explanation about ES6 modules and inline handlers,
                // see <https://stackoverflow.com/a/59539045>
                <Script type_="module">r#"
                    import { Graphviz } from "https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js";
                    const graphviz = await Graphviz.load();
                    window.graphviz = graphviz;
                "#
                </Script>
            </div>
        </Suspense>
    }
}
