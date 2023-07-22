#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use dot_ix::{app::*, fileserv::file_and_error_handler};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    // Setting `get_configuration(None)` means we'll be using cargo-leptos's env
    // values.
    //
    // For deployment these variables are:
    //
    // <https://github.com/leptos-rs/dot_ix#executing-a-server-on-a-remote-machine-without-the-toolchain>
    //
    // Alternately a file can be specified such as `Some("Cargo.toml")`
    //
    // The file would need to be included with the executable when moved to
    // deployment.
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "csr")]
pub fn main() {
    use leptos::*;

    use dot_ix::app::App;

    // client-side main function
    // so that this can work with e.g. Trunk for a purely client-side app
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}

#[cfg(all(not(feature = "ssr"), not(feature = "csr")))]
pub fn main() {
    // no client-side main function
    // see lib.rs for hydration function for a non-csr app
}
