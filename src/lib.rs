mod routes;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use routes::*;

/// Register app routes for our app
pub fn create_app(db: sled::Db) -> Router {
    Router::new()
        .route("/", get(health))
        .route("/shorten", post(shorten))
        .route("/redirect/:id", get(redirect))
        .layer(Extension(db))
}
