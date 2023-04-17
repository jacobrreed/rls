extern crate url;
use url::{Url};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
struct UrlRes {
    url: String,
    shortUrl: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new().route("/", post(generate_url)).layer(cors);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

async fn generate_url(Json(payload): Json<CreateUrl>) -> impl IntoResponse {
    // check if url is valid
    let url = Url::parse(&payload.url).unwrap();
    let url_res = UrlRes {
        url: url.to_string(),
        shortUrl: String::from("https://neonvoid.io/{}"),
    };

    (StatusCode::CREATED, Json(url_res))
}
