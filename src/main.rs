use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, delete},
    Router, Json,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
#[derive(Deserialize)]
struct Page {
    number: u32,
}

#[derive(Deserialize)]
struct Item {
    title: String
}

async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted User".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("Usuário não pode ser deletado.".to_string())
    } else {
        Ok(())
    }
}


async fn add_item(Json(item): Json<Item>) -> String {
    format!("Item adicionado: {}", item.title)
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
        },
        User {
            id: 2,
            name: "John".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/add-item", post(add_item))
        .route("/item/:id", get(show_item))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
