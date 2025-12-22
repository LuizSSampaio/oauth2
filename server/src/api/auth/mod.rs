use axum::{Router, routing::post};

mod register;

pub fn app() -> Router {
    Router::new().route("/register", post(register::register))
}
