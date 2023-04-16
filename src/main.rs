extern crate redis;
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use redis::Commands;
use serde::Deserialize;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
struct Url {
    id: u64,
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
    let url = Url {
        url: payload.url,
        shortUrl: String::from("https://neonvoid.io/{}", 1),
    };

    (StatusCode::CREATED, Json(url))
}

fn set_url(url: &str, surl: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;
    let _: () = con.set(surl, url)?;
    Ok(())
}
