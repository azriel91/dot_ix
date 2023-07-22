#![allow(non_snake_case)] // Components are all PascalCase.

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use self::{
    dot_svg::DotSvg,
    error_template::{AppError, ErrorTemplate},
    info_graph::InfoGraph,
};

mod dot_svg;
mod error_template;
mod info_graph;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dot_ix.css" />
        <Stylesheet id="fonts" href="/fonts/fonts.css" />
        <Title text="dot_ix: Interactive dot graphs" />

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-xl">"✒️ dot_ix: Interactive Dot graph"</h1>
        <InfoGraph />
        <div class="
            border
            border-amber-300
            bg-gradient-to-b from-amber-100 to-amber-200
            my-2
            p-2
            rounded
            ">
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
                "This is a "<i>"very early"</i>" frontend prototype for the "
                <a
                    class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                    href="https://peace.mk"
                >
                    "Peace"
                </a>
                " automation framework"
            </p>
            <p>
                <b>"Known Issues:"</b>
                <ol class="list-disc mx-4">
                // The following is true for `csr`, not `ssr`.
                <li>"On load, clicking tags doesn't highlight nodes until you edit the text."</li>
                <li>"Nodes with tags sometimes aren't highlighted when the tags are clicked."<br />
                    "This is because graphviz generates nodes in a non-deterministic order, and the "
                    <a
                        class="text-sky-600 hover:text-sky-400 active:text-sky-300"
                        href="https://developer.mozilla.org/en-US/docs/Web/CSS/General_sibling_combinator"
                    >
                        "General sibling combinator"
                    </a>
                    " requires the tag-node (cluster) to appear before the node-node."
                </li>
                <li>"Nodes with multiple tags only apply either the stroke, or fill, or neither."<br />
                    "e.g. click on Tag 1 and Tag 2, and observe node C's fill / stroke."<br />
                    "This may be related to multiple CSS attribute rules across multiple classes and precedence."<br />
                    "But I don't know 🥲. Please help solve it if you can."<br />
                </li>
                </ol>
            </p>
        </div>
    }
}
