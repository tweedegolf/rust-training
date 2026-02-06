use axum::{body::Body, extract::Request, response::Response};

pub async fn my_handler(_req: Request<Body>) -> Response<Body> {
    Response::builder()
        .body(Body::from("Hello, world!"))
        .unwrap()
}
