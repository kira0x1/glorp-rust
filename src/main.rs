use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::{routing::get, Router};
use std::fmt::Debug;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_http::{cors::{CorsLayer}};
use tracing::{info, Level};

//noinspection HttpUrlsUsage
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let cors = CorsLayer::new();

    let static_files = ServeDir::new("static")
        .fallback(ServeFile::new("static/not_found.html"));

    let app = Router::new()
        .route("/", get(index_handler))
        .fallback(|| async { AppError::NotFound })
        .nest_service("/static", static_files)
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors.clone())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(Error::Bind)?;

    if let Ok(addr) = listener.local_addr() {
        info!("Listening on http://{addr}");
    }

    axum::serve(listener, app).await.map_err(Error::Run)
}

#[derive(displaydoc::Display, Debug, thiserror::Error)]
enum Error {
    /// could not bind socket
    Bind(#[source] std::io::Error),
    /// could not run server
    Run(#[source] std::io::Error),
}


/// This enum contains any error that could occur while handling an incoming request.
///
/// In a real application you would most likely have multiple error sources, e.g. database errors,
#[derive(Debug, displaydoc::Display, thiserror::Error)]
enum AppError {
    /// not found
    NotFound,
    /// could not render template
    Render(#[from] askama::Error),
}


/// This is your error handler
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // It uses an askama template to display its content.
        // The member `lang` is used by "_layout.html" which "error.html" extends. Even though it
        // is always the fallback language English in here, "_layout.html" expects to be able to
        // access this field, so you have to provide it.
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            err: AppError,
        }

        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl {
            err: self,
        };
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}

async fn index_handler() -> Result<impl IntoResponse, AppError> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {}
    let template = Tmpl {};

    Ok(Html(template.render()?))
}