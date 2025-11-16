use axum_tonic_demo::pb::{
    AddTodoRequest, AddTodoResponse, CompleteTodoRequest, CompleteTodoResponse, Empty, TodoItem,
    todo_client::TodoClient,
};
use clap::Parser;
use tabled::{Table, Tabled};
use tonic::Request;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Add a new todo")]
    add: Option<String>,
    #[arg(short, long, help = "Get all todos")]
    list: bool,
    #[arg(short, long, help = "Complete the specified todo")]
    complete: Option<i64>,
}

#[derive(Tabled)]
struct Task {
    id: i64,
    description: String,
    done: bool,
}

impl From<TodoItem> for Task {
    fn from(
        TodoItem {
            id,
            description,
            done,
        }: TodoItem,
    ) -> Self {
        Self {
            id,
            description,
            done,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = TodoClient::connect("http://127.0.0.1:3000").await?;

    let arg = Cli::parse();

    if let Some(description) = arg.add {
        let request = Request::new(AddTodoRequest { description });
        let AddTodoResponse { id } = client.add_todo_item(request).await?.into_inner();

        println!("A new todo was successfully added. The id is {id}");
    }

    if let Some(id) = arg.complete {
        let request = Request::new(CompleteTodoRequest { id });
        let CompleteTodoResponse { completed } = client.complete_todo(request).await?.into_inner();

        println!(
            "Completed is {}",
            if completed { "successful" } else { "failure" }
        );
    }

    if arg.list {
        let request = Request::new(Empty::default());
        let response = client.get_todo_list(request).await?;
        let todos = response.into_inner();
        let tasks = todos
            .list
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Task>>();
        let table = Table::new(tasks);
        println!("{table}");
    }

    Ok(())
}
