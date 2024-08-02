use axum::{
    body::Body,
    Extension,
    middleware::{self, Next},
    extract::{Path, Query},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    Router,
    routing::{delete, get, post}};
use axum::http::Request;
use serde::{Deserialize, Serialize};
use sqlx::{Row, MySqlPool};
use serde_json::json;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String

}
#[derive(Deserialize)]
struct Page {
    number: u32,
}

#[derive(Deserialize)]
struct Item {
    title: String
}

async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Recebendo uma requisição para {}", req.uri());
    
    next.run(req).await
}

async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted User".into(),
            email: "".into()
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

async fn list_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(), 
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

#[tokio::main]
async fn main() {
    let database_url = "mysql://root:password@localhost:3306/explore_axum_db";
    let pool = MySqlPool::connect(&database_url)
        .await.expect("Não foi possível conectar ao banco de dados");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/add-item", post(add_item))
        .route("/item/:id", get(show_item))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .layer(Extension(pool))
        .layer(middleware::from_fn(logging_middleware))
        ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
