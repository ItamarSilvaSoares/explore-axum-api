use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, Json,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}
#[derive(Deserialize)]
struct Page {
    number: u32,
}

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} na pagina {}", id, page.number)
}

async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("Usuário criado com sucesso"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/item/:id", get(show_item))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
