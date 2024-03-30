use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use spiegel_server::{get_closest_color, init};

/// serves images from memory
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    init();
    let app = Router::new().route("/api/color/:rgb_hex", get(fetch_nearest_color));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("started server on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn fetch_nearest_color(
    Path(rgb_hex): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if rgb_hex.len() != 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "input should be color hex, eg AA11CC".into(),
        ));
    }
    let closest = get_closest_color(&rgb_hex);

    let headers = [(header::CONTENT_TYPE, "image/jpeg")];
    Ok((headers, closest.image))
}
