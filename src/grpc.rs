use crate::{
    database::service::{self, Todo as TodoPO},
    pb::{
        AddTodoRequest, AddTodoResponse, CompleteTodoRequest, CompleteTodoResponse, Empty,
        TodoItem, TodoList, todo_server::Todo,
    },
};
use futures::StreamExt;
use sqlx::SqlitePool;
use tonic::{Code, Request, Response, Status};

pub struct TodoGrpcService {
    pool: SqlitePool,
}

impl TodoGrpcService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
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

#[tonic::async_trait]
impl Todo for TodoGrpcService {
    async fn get_todo_list(&self, _request: Request<Empty>) -> Result<Response<TodoList>, Status> {
        let mut stream = match service::stream_todos(&self.pool).await {
            Ok(stream) => stream,
            Err(_err) => return Err(Status::new(Code::Internal, "The data cannot be obtained.")),
        };
        let mut todos = Vec::<TodoItem>::new();
        while let Some(Ok(todo)) = stream.next().await {
            todos.push(todo.into());
        }

        Ok(Response::new(TodoList { list: todos }))
    }

    async fn add_todo_item(
        &self,
        request: Request<AddTodoRequest>,
    ) -> Result<Response<AddTodoResponse>, Status> {
        let AddTodoRequest { description } = request.into_inner();
        let id = match service::add_todo(&self.pool, description).await {
            Ok(id) => id,
            Err(_err) => return Err(Status::new(Code::Unimplemented, "Task insertion failed.")),
        };
        Ok(Response::new(AddTodoResponse { id }))
    }

    async fn complete_todo(
        &self,
        request: Request<CompleteTodoRequest>,
    ) -> Result<Response<CompleteTodoResponse>, Status> {
        let CompleteTodoRequest { id } = request.into_inner();
        let completed = match service::complete_todo(&self.pool, id).await {
            Ok(completed) => completed,
            Err(_err) => return Err(Status::new(Code::Unimplemented, "Task completed failed.")),
        };
        Ok(Response::new(CompleteTodoResponse { completed }))
    }
}
