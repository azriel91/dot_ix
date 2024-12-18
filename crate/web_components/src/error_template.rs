use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::{
    component,
    control_flow::For,
    error::Errors,
    prelude::{ElementChild, GetUntracked, RwSignal},
    view, IntoView,
};
use thiserror::Error;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error(
        "Route Not Found: `{path}`. Check that the site prefix is correctly set and routed to."
    )]
    RouteNotFound {
        /// The path that isn't routed.
        path: String,
    },
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::RouteNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<RwSignal<Errors>>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = outside_errors
        .or(errors)
        .expect("No Errors found and we expected errors!");

    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! { if #[cfg(feature="ssr")] {
        let response = leptos::context::use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }}

    view! {
        <h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each= move || {errors.clone().into_iter().enumerate()}
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children= move |error| {
                let error_string = error.1.to_string();
                let error_code= error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}
