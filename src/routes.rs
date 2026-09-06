use axum::{Json, Router, extract::State, routing::get};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::db::{self, DbClient};
use crate::models::{Employee, InsertResult, NewEmployee};

pub type AppState = Arc<Mutex<DbClient>>;

pub fn create_router(db_client: DbClient) -> Router {
    let state: AppState = Arc::new(Mutex::new(db_client));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/employees", get(list_handler).post(insert_handler))
        .with_state(state)
        .layer(cors)
}

async fn list_handler(State(state): State<AppState>) -> Json<Vec<Employee>> {
    let mut client = state.lock().await;
    let employees = db::list_employees(&mut client).await;
    Json(employees)
}

async fn insert_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewEmployee>,
) -> Json<InsertResult> {
    let mut client = state.lock().await;
    let result = db::insert_employee(&mut client, &payload.name, payload.salary).await;
    Json(result)
}
