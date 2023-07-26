use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/live", get(health))
}

async fn health() -> &'static str {
    "Ok"
}
