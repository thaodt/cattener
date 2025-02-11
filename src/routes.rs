use axum::{debug_handler, extract::Json, extract::Path, response::Redirect, Extension};
use log::info;
use serde::Deserialize;

/// Payload for shortening a URL
#[derive(Debug, Deserialize)]
pub struct Payload {
    url: String,
}

/// Health check endpoint
#[debug_handler]
pub async fn health() -> String {
    "Hello, developer.".to_string()
}

/// Shorten a URL
#[debug_handler]
pub async fn shorten(Extension(db): Extension<sled::Db>, Json(payload): Json<Payload>) -> String {
    info!("{:?}", payload);
    let mut uuid = nanoid::nanoid!(8);
    while db.contains_key(&uuid).unwrap() {
        uuid = nanoid::nanoid!(8);
    }
    let url_as_bytes = payload.url.as_bytes();
    db.insert(&uuid, url_as_bytes).unwrap();
    info!("key: {}, value: {:?}", uuid, url_as_bytes);
    assert_eq!(&db.get(uuid.as_bytes()).unwrap().unwrap(), url_as_bytes);
    uuid
}

///  Redirect to full URL
#[debug_handler]
pub async fn redirect(Path(id): Path<String>, Extension(db): Extension<sled::Db>) -> Redirect {
    match &db.get(&id).unwrap() {
        Some(url) => {
            let url = String::from_utf8(url.to_vec()).unwrap();
            info!("URL found: {:#?}", url);
            Redirect::to(&url)
        }
        None => {
            info!("URL not found.");
            Redirect::to("/")
        }
    }
}
