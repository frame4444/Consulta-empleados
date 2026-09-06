mod db;
mod models;
mod routes;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_client = db::connect_db().await;
    let app = Router::new().route("/", get(saludo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to open port:3000");

    println!("Server running at http://localhost:3000");

    axum::serve(listener, app).await.expect("server crashed");
}

async fn saludo() -> &'static str {
    "Hola, mundo!"
}
