use axum::{Json, response::IntoResponse};
use serde::Deserialize;

pub async fn register(Json(payload): Json<Request>) -> impl IntoResponse {}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub email: String,
}
