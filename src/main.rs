use std::net::SocketAddr;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use tracing::debug;

use self::handler::{health, torrentsapi};

mod database;
mod handler;
mod torrent;
mod tvdb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest("/health", health::router())
        .nest("/", torrentsapi::router().await?)
        .fallback(handle_not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    debug!("Listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_not_found(req: Request<Body>) -> impl IntoResponse {
    debug!("Got request to unknown route: {}", req.uri());
    (StatusCode::NOT_FOUND, "nothing to see here")
}
