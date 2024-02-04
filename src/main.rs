#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use dot_ix::{app::*, fileserv::file_and_error_handler};
    use leptos::{logging::log, *};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use log4rs::{
        append::console::{ConsoleAppender, Target},
        config::Appender,
        filter::threshold::ThresholdFilter,
    };

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let log4rs_config = log4rs::config::Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stderr")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();

    let _log4rs_handle = log4rs::init_config(log4rs_config).expect("Failed to set logger");

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
    let socket_addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    // build our application with a route
    let router = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to listen on {socket_addr}. Error: {e}"));
    log!("listening on http://{}", &socket_addr);
    axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "csr")]
pub fn main() {
    use leptos::{logging::log, *};

    use dot_ix::app::App;

    // client-side main function
    // so that this can work with e.g. Trunk for a purely client-side app
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! {  <App /> }
    });
}

#[cfg(all(not(feature = "ssr"), not(feature = "csr")))]
pub fn main() {
    // no client-side main function
    // see lib.rs for hydration function for a non-csr app
}
