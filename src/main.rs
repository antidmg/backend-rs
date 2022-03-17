use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    println!("Server started on port 8080");
    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().route("/status", get(status))
}

async fn status() -> impl IntoResponse {
    println!("Status OK");
    StatusCode::OK
}
