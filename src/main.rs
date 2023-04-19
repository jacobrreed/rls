extern crate url;
use harsh::Harsh;
use url::{Url};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Serialize)]
struct UrlRes {
    url: String,
}

const SALT: &str = "rls";

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new().route("/", post(generate_url)).layer(cors);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

// post request that handles generating a short url
async fn generate_url(Json(payload): Json<UrlRes>) -> (axum::http::StatusCode, axum::Json<UrlRes>) {

}


// function that generates a short url hash given a string
fn generate_hash(url: &str) -> String {
  let harsh = Harsh::builder().salt(SALT).build().unwrap();
  return harsh.encode(&[url.len() as u64]);
}

// create api that takes POST request
// post request takes url
// -> check if URL exists in db (lookup hash)
//     if it does:
//       return 200 with short url
//     if it doesnt:
//       generate a hash for short url
//       store hash in db with long url as value
//       return 201 (create) with short url
// get request:
//   takes hash from url "example.com/THISISTHEHASH"
//   lookup hash in db
//   redirect to long url (or return) (301?)
