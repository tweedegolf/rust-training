use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Specify available routes
    let app = Router::new().route("/", get("Hello, world!"));

    // Serve the website on localhost
    let addr = "0.0.0.0:7000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Serving on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
