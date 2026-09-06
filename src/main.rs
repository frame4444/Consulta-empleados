mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_client = db::connect_db().await;
    let app = routes::create_router(db_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("could not bind to port 3000");

    println!("Server running at http://localhost:3000");
    println!("https://localhost:3000/index.html");

    axum::serve(listener, app).await.expect("server crashed");
}
