use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize, Tabled)]
struct Task {
    id: i64,
    description: String,
    done: bool,
}

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:3000/todo/all";
    let response = reqwest::get(url).await.unwrap();

    let body = response.text().await.unwrap();

    let todos = serde_json::from_str::<Vec<Task>>(&body).unwrap();

    let table = Table::new(todos);

    println!("{table}");
}
