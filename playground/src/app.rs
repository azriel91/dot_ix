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

    // I *think* this is necessary because the `TrailingSlash` logic doesn't work
    // when the app is served through a static site, e.g. GitHub pages.
    //
    // When the site prefix is empty, we cannot mount both `""` and `"/"`, because
    // `axum` panics, as both paths map to the root resource.
    //
    // When the site prefix is *not* empty, then we have to mount both
    // `{site_prefix}` and `{site_prefix}/`, because when served through a static
    // site, we don't have server site logic to apply the `TrailingSlash` logic.
    // That is, I'm assuming `TrailingSlash` is applied through `"ssr"`, and not
    // through client side routing.
    if site_prefix.is_empty() {
        view! {
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href=stylesheet_path />
            <Stylesheet id="fonts" href=fonts_path />
            <Title text="dot_ix: Interactive dot graphs" />

            // content for this welcome page
            <Router
                fallback=|| RouterFallback.into_view()
            >
                <main>
                    <Routes>
                        <Route
                            path=""
                            trailing_slash=TrailingSlash::Drop
                            view=|| view! { <HomePage/> }
                        />
                    </Routes>
                </main>
            </Router>
        }
    } else {
        view! {
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href=stylesheet_path />
            <Stylesheet id="fonts" href=fonts_path />
            <Title text="dot_ix: Interactive dot graphs" />

            // content for this welcome page
            <Router
                fallback=|| RouterFallback.into_view()
            >
                <main>
                    <Routes>
                        <Route
                            path=site_prefix
                            trailing_slash=TrailingSlash::Exact
                            view=|| view! { <HomePage/> }
                        />
                        <Route
                            path=format!("{site_prefix}/")
                            trailing_slash=TrailingSlash::Exact
                            view=|| view! { <HomePage/> }
                        />
                    </Routes>
                </main>
            </Router>
        }
    }
}

/// Renders the home page of your application.
#[component]
fn RouterFallback() -> impl IntoView {
    let route_context = leptos_router::use_route();
    let path = route_context.path();
    let path_unresolved = route_context.resolve_path("").is_none();

    let mut outside_errors = Errors::default();
    if path_unresolved {
        outside_errors.insert_with_default_key(AppError::RouteNotFound { path });
    }

    let outside_errors = create_rw_signal(outside_errors);
    view! {
        <Show
            when=move || path_unresolved
        >
            <ErrorTemplate outside_errors />
        </Show>
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
                    <li>"The Flex Diagram is still experimental, and the arrows don't \
                        receive styling."</li>
                    <li>"If you'd like to contribute to the development of this library, \
                        I'd be super delighted, since I'm not a web-dev™️ 🙃."</li>
                    </ol>
                </p>
            </div>
        </div>
    }
}
