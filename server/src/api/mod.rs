use axum::Router;

mod auth;

pub fn app() -> Router {
    Router::new().nest("/auth", auth::app())
}
