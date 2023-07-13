use leptos::*;

#[server(DotSvgRender, "/api")]
pub async fn dot_svg(dot_src: String) -> Result<(String, String), ServerFnError> {
    use std::process::Stdio;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut dot_process = tokio::process::Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn dot command");

    if let Some(mut stdin) = dot_process.stdin.take() {
        stdin
            .write_all(dot_src.as_bytes())
            .await
            .map_err(|error| ServerFnError::ServerError(format!("{error}")))?;
    }

    let mut dot_svg = String::with_capacity(dot_src.len());
    if let Some(mut stdout) = dot_process.stdout.take() {
        stdout
            .read_to_string(&mut dot_svg)
            .await
            .map_err(|error| ServerFnError::ServerError(format!("{error}")))?;
    }

    let mut dot_stderr = String::new();
    if let Some(mut stderr) = dot_process.stderr.take() {
        stderr
            .read_to_string(&mut dot_stderr)
            .await
            .map_err(|error| ServerFnError::ServerError(format!("{error}")))?;
    }

    dot_process
        .wait()
        .await
        .map_err(|error| ServerFnError::ServerError(format!("{dot_stderr}{error}")))?;

    Ok((dot_svg, dot_stderr))
}

/// Renders a graphviz graph as an SVG.
#[component]
pub fn DotSvg(cx: Scope, dot_src: ReadSignal<Option<String>>) -> impl IntoView {
    let dot_svg_and_error_resource = create_resource(cx, dot_src, |dot_src| async move {
        if let Some(dot_src) = dot_src {
            if !dot_src.is_empty() {
                match dot_svg(dot_src).await {
                    Ok((dot_svg, error_text)) => (dot_svg, error_text),
                    Err(error) => (String::from(""), format!("{error}")),
                }
            } else {
                (String::from(""), String::from(""))
            }
        } else {
            (String::from(""), String::from(""))
        }
    });

    view! { cx,
        <Suspense
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            <h2>"Graph"</h2>
            {move || {
                dot_svg_and_error_resource.read(cx)
                    .map(|(dot_svg, error_text)| view! {
                        cx,
                        <div>
                            <div inner_html=dot_svg />

                            <div class={
                                let error_text_empty = error_text.is_empty();
                                move || {
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
                            >{error_text}</div>
                        </div>
                    })
            }}
        </Suspense>
    }
}
