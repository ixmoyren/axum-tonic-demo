use crate::database::{service, service::Todo as TodoPO};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use utoipa::{
    Modify, OpenApi, ToSchema,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::{router::OpenApiRouter, routes};

const TODO_TAG: &str = "todo";

pub fn todo_router(pool: SqlitePool) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list))
        .routes(routes!(add))
        .routes(routes!(complete))
        .with_state(pool)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = TODO_TAG, description = "Todo items management API")
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_api"))),
            )
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AddTodo {
    description: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TodoItem {
    id: i64,
    description: String,
    done: bool,
}

impl From<TodoPO> for TodoItem {
    fn from(
        TodoPO {
            id,
            description,
            done,
        }: TodoPO,
    ) -> Self {
        Self {
            id,
            description,
            done,
        }
    }
}

/// 获取全部的待办
#[utoipa::path(
    get,
    path = "/all",
    tag = TODO_TAG,
    responses(
        (status = 200, description = "List all todo items", body = [TodoItem])
    )
)]
pub async fn list(State(pool): State<SqlitePool>) -> Json<Vec<TodoItem>> {
    let mut stream = service::stream_todos(&pool).await.unwrap();
    let mut todos = Vec::<TodoItem>::new();
    while let Some(Ok(todo)) = stream.next().await {
        todos.push(todo.into());
    }
    Json(todos)
}

/// 新增一个待办
#[utoipa::path(
    post,
    path = "/add",
    tag = TODO_TAG,
    request_body(content = AddTodo, description = "Add new one todo item", content_type = "application/json"),
    responses(
        (status = 200, description = "Create new one todo item", body = i64)
    ),
)]
pub async fn add(
    State(pool): State<SqlitePool>,
    Json(AddTodo { description }): Json<AddTodo>,
) -> Json<i64> {
    let id = service::add_todo(&pool, description).await.unwrap();
    Json(id)
}

/// 完成一个待办
#[utoipa::path(
    post,
    path = "/complete/{id}",
    tag = TODO_TAG,
    responses(
        (status = 200, description = "The todo item has marked done", body = [bool])
    ),
    params(
        ("id" = i64, Path, description = "Todo item id")
    )
)]
pub async fn complete(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> Json<bool> {
    let completed = service::complete_todo(&pool, id).await.unwrap();
    Json(completed)
}
