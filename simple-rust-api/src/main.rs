use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

// This defines the structure of the JSON we expect to get from the internet
#[derive(Serialize, Deserialize, Debug)]
struct MockUser {
    id: u32,
    name: String,
    username: String,
    email: String,
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(health_check))
        .route("/proxy", get(fetch_external_data));

    // Bind to 0.0.0.0 so it's accessible outside a Docker container later
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}

// Basic health check endpoint
async fn health_check() -> &'static str {
    "API is up and running!"
}

// Outbound internet call handler
async fn fetch_external_data() -> Result<Json<MockUser>, String> {
    let url = "https://jsonplaceholder.typicode.com/users/1";
    
    // Make the outbound REST call to the internet
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("Failed to fetch from internet: {}", err))?;

    // Parse the JSON payload into our Rust struct
    let user = response
        .json::<MockUser>()
        .await
        .map_err(|err| format!("Failed to parse JSON: {}", err))?;

    // Return the data wrapped inside Axum's Json extractor
    Ok(Json(user))
}