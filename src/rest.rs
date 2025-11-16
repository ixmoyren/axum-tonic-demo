use crate::database::service;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub fn todo_router(pool: SqlitePool) -> Router<()> {
    Router::new()
        .route("/all", get(list))
        .route("/add", post(add))
        .route("/complete/{id}", post(complete))
        .with_state(pool)
}

#[derive(Deserialize, Serialize)]
pub struct AddTodo {
    description: String,
}

pub async fn list(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let todos = service::list_todos(&pool).await.unwrap();
    (StatusCode::OK, Json(todos))
}

pub async fn add(
    State(pool): State<SqlitePool>,
    Json(AddTodo { description }): Json<AddTodo>,
) -> impl IntoResponse {
    let id = service::add_todo(&pool, description).await.unwrap();
    (StatusCode::OK, Json(id))
}

pub async fn complete(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    let completed = service::complete_todo(&pool, id).await.unwrap();
    (StatusCode::OK, Json(completed))
}
