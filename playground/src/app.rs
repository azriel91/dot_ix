#![allow(non_snake_case)] // Components are all PascalCase.

use std::time::Duration;

use dot_ix::web_components::{AppError, ErrorTemplate};
use leptos::{
    component,
    control_flow::Show,
    either::Either,
    error::Errors,
    prelude::{signal, ClassAttribute, ElementChild, Get, RwSignal, Signal},
    view, IntoView,
};
use leptos_meta::{provide_meta_context, Meta, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress},
    StaticSegment,
};

use self::{info_graph::InfoGraph, tabs::TabLabel, text_editor::TextEditor};

mod info_graph;
mod tabs;
mod text_editor;

#[cfg(feature = "ssr")]
use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::{GlobalAttributes, LeptosOptions},
};
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;

/// Whether to only draw the diagram and hide the text boxes.
#[cfg(target_arch = "wasm32")]
const QUERY_PARAM_DIAGRAM_ONLY: &str = "diagram_only";

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Top level playground application, but is wrapped in a `shell` for `"ssr"`.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");
    let stylesheet_path = format!("{site_prefix}/pkg/dot_ix.css");
    let fonts_path = format!("{site_prefix}/fonts/fonts.css");

    let (is_routing, set_is_routing) = signal(false);

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
        let view = view! {
            <AppMeta stylesheet_path fonts_path />
            <Router set_is_routing>
                <div class="routing-progress">
                    <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
                </div>
                <main>
                    <Routes fallback=|| RouterFallback.into_view()>
                        <Route
                            path=leptos_router_macro::path!("")
                            // trailing_slash=TrailingSlash::Drop // leptos 0.7 WIP
                            view=HomePage
                        />
                    </Routes>
                </main>
            </Router>
        };
        Either::Left(view)
    } else {
        let view = view! {
            <AppMeta stylesheet_path fonts_path />
            <Router set_is_routing>
                <div class="routing-progress">
                    <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
                </div>
                <main>
                    <Routes fallback=|| RouterFallback.into_view()>
                        <Route
                            path=StaticSegment(site_prefix)
                            // trailing_slash=TrailingSlash::Exact // leptos 0.7 WIP
                            view=HomePage
                        />
                        <Route
                            path=(StaticSegment(site_prefix), StaticSegment("/"))
                            // trailing_slash=TrailingSlash::Exact // leptos 0.7 WIP
                            view=HomePage
                        />
                    </Routes>
                </main>
            </Router>
        };

        Either::Right(view)
    }
}

#[component]
pub fn AppMeta(stylesheet_path: String, fonts_path: String) -> impl IntoView {
    view! {
        <GoogleAnalyticsHeader />
        <GoogleTagManagerHeader />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href=stylesheet_path />
        <Stylesheet id="fonts" href=fonts_path />
        <Title text="dot_ix: Interactive dot graphs" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
    }
}

#[component]
pub fn GoogleAnalyticsHeader() -> impl IntoView {
    view! {
        <Script async_="true" src="https://www.googletagmanager.com/gtag/js?id=G-QWXYFJ1NED"></Script>
        <Script>
            "\
            window.dataLayer = window.dataLayer || [];\n\
            function gtag(){dataLayer.push(arguments);}\n\
            gtag('js', new Date());\n\
            gtag('config', 'G-QWXYFJ1NED');\n\
            "
        </Script>
    }
}

#[component]
pub fn GoogleTagManagerHeader() -> impl IntoView {
    view! {
        <Script>
            "\
            (function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':\n\
            new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],\n\
            j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=\n\
            'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);\n\
            })(window,document,'script','dataLayer','GTM-W2485ZNP');\n\
            "
        </Script>
    }
}

// This is normally meant to be inserted into the body of the document, as a
// fallback when `<script>` tags are not supported. However, it breaks hydration
// when leptos is used in SSR mode, since hydration requires element counts to
// be in sync on both the server side and client side.
//
// Tracked on [leptos#3360](https://github.com/leptos-rs/leptos/issues/3360).
//
// ```rust,ignore
// #[component]
// pub fn GoogleTagManagerBody() -> impl IntoView {
//     view! {
//         <noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-W2485ZNP"
//         height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>
//     }
// }
// ```

/// Renders the home page of your application.
#[component]
fn RouterFallback() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = location.pathname;
    // let path_unresolved = route_context.resolve_path("").is_none();
    let path_unresolved = false; // TODO

    let mut outside_errors = Errors::default();
    if path_unresolved {
        outside_errors.insert_with_default_key(AppError::RouteNotFound {
            path: pathname.get(),
        });
    }

    let outside_errors = RwSignal::new(outside_errors);
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
    let (diagram_only, _set_diagram_only) =
        leptos_router::hooks::query_signal::<bool>("diagram_only");

    let diagram_only = Signal::derive(move || diagram_only.get().unwrap_or(false));

    let main_div_classes = move || {
        if diagram_only.get() {
            "font-sans"
        } else {
            "font-sans lg:p-4"
        }
    };

    view! {
        <div class=main_div_classes>
            <InfoGraph diagram_only />
        </div>
    }
}
