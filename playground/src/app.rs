#![allow(non_snake_case)] // Components are all PascalCase.

use dot_ix::web_components::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use self::info_graph::InfoGraph;

mod info_graph;

/// Whether to only draw the diagram and hide the text boxes.
#[cfg(target_arch = "wasm32")]
const QUERY_PARAM_DIAGRAM_ONLY: &str = "diagram_only";

/// Sets the info graph src using logic purely executed on the client side.
///
/// This is for a pure client side rendered app, so updating a signal within
/// `create_effect` is safe.
#[cfg(target_arch = "wasm32")]
fn diagram_only_init(set_diagram_only: WriteSignal<bool>) {
    use web_sys::Url;

    create_effect(move |_| {
        if let Some(url_search_params) = web_sys::window().map(|window| {
            Url::new(&String::from(window.location().to_string()))
                .expect("Expected URL to be valid.")
                .search_params()
        }) {
            let diagram_only = url_search_params
                .get(QUERY_PARAM_DIAGRAM_ONLY)
                .and_then(|diagram_only_str| serde_yaml::from_str::<bool>(&diagram_only_str).ok())
                .unwrap_or(false);
            set_diagram_only.set(diagram_only);
        } else {
            set_diagram_only.set(false);
        }
    });
}

/// Top level playground application.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");
    let stylesheet_path = format!("{site_prefix}/pkg/dot_ix.css");
    let fonts_path = format!("{site_prefix}/fonts/fonts.css");

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href=stylesheet_path />
        <Stylesheet id="fonts" href=fonts_path />
        <Title text="dot_ix: Interactive dot graphs" />

        // content for this welcome page
        <Router
            fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
                .into_view()
            }
        >
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (diagram_only, set_diagram_only) = create_signal(true);

    #[cfg(target_arch = "wasm32")]
    diagram_only_init(set_diagram_only);

    let _set_diagram_only = set_diagram_only;

    let main_div_classes = move || {
        if diagram_only.get() { "" } else { "md:p-12" }
    };
    let disclaimer_classes = move || {
        if diagram_only.get() {
            "hidden"
        } else {
            "
            border
            border-amber-300
            bg-gradient-to-b from-amber-100 to-amber-200
            my-2
            p-2
            rounded
            "
        }
    };

    view! {
        <div class=main_div_classes>
            <InfoGraph diagram_only=diagram_only />
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
                    <li>"Nodes with multiple tags only apply either the stroke, or fill, or neither."<br />
                        "e.g. click on Tag 1 and Tag 2, and observe node C's fill / stroke."<br />
                        "This may be related to multiple CSS attribute rules across multiple classes and precedence."<br />
                        "But I don't know 🥲. Please help solve it if you can."<br />
                    </li>
                    </ol>
                </p>
            </div>
        </div>
    }
}
