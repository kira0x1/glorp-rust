mod config;
mod pages;
mod parser;

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::{Router, routing::get};
use pages::IconBox;
use pages::IndexTmpl;
use rand::seq::IndexedRandom;
use std::fmt::Debug;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;

//noinspection HttpUrlsUsage
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = parser::parse_args();

    println!("version {:?}", "GLORP VERSION MEOW");
    println!("dev: {:?}, trace: {:?}", args.is_dev, args.trace_level);

    tracing_subscriber::fmt()
        .with_max_level(args.trace_level)
        .init();

    let cors = CorsLayer::new();

    let static_files = ServeDir::new("static")
        .fallback(ServeFile::new("static/not_found.html"));

    let cow_txt = ServeFile::new("static/cow.txt");

    let app = Router::new()
        .route("/", get(index_handler))
        .fallback(|| async { AppError::NotFound })
        .nest_service("/static", static_files)
        .nest_service("/cow.txt", cow_txt)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors.clone()),
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
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            err: AppError,
        }

        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl { err: self };
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}

async fn index_handler() -> Result<impl IntoResponse, AppError> {
    // pick random glorp message
    let x = config::GLORP_MESSAGES.choose(&mut rand::rng()).unwrap();

    let args = parser::parse_args();

    if args.is_dev {
        println!("chose message: {:?}", x);
    }

    let data: Vec<IconBox> = vec![IconBox {
        message: "i have a bow",
        icon: "/static/images/glorpBow.png",
    }];

    let template = IndexTmpl {
        globe_message: x,
        glorp_status: data,
    };

    Ok(Html(template.render()?))
}
