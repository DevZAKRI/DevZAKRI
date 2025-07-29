// Rust Web Server Example for JavaScript/Node.js Developers
// This is similar to Express.js but with Rust's type safety and performance
// Run with: cargo run

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

// Data structures (similar to TypeScript interfaces)
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

#[derive(Deserialize)]
struct QueryParams {
    #[serde(default)]
    limit: Option<u32>,
    #[serde(default)]
    offset: Option<u32>,
}

// Application state (similar to global variables in Node.js apps)
type AppState = Arc<Mutex<HashMap<u32, User>>>;

#[tokio::main]
async fn main() {
    println!("🦀 Starting Rust Web Server (similar to Express.js)");
    
    // Initialize in-memory database (like a global variable in JS)
    let users_db: AppState = Arc::new(Mutex::new(HashMap::new()));
    
    // Add some sample data
    {
        let mut db = users_db.lock().unwrap();
        db.insert(1, User {
            id: 1,
            name: "DevZAKRI".to_string(),
            email: "3tern4llord@gmail.com".to_string(),
            age: 25,
        });
        db.insert(2, User {
            id: 2,
            name: "Rust Developer".to_string(),
            email: "rust@example.com".to_string(),
            age: 30,
        });
    }
    
    // Define routes (similar to Express routes)
    let app = Router::new()
        // GET / - Welcome route
        .route("/", get(welcome))
        
        // GET /health - Health check (common in Node.js apps)
        .route("/health", get(health_check))
        
        // REST API routes for users
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))
        
        // Static route examples
        .route("/api/info", get(server_info))
        
        // Add CORS middleware (like in Express)
        .layer(CorsLayer::permissive())
        
        // Add state to all routes
        .with_state(users_db);

    // Start server (similar to app.listen() in Express)
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📡 Try these endpoints:");
    println!("  GET  /                 - Welcome message");
    println!("  GET  /health           - Health check");
    println!("  GET  /api/users        - Get all users");
    println!("  GET  /api/users/1      - Get user by ID");
    println!("  POST /api/users        - Create new user");
    println!("  PUT  /api/users/1      - Update user");
    println!("  DELETE /api/users/1    - Delete user");
    println!("  GET  /api/info         - Server info");
    
    axum::serve(listener, app).await.unwrap();
}

// Route handlers (similar to Express route handlers)

async fn welcome() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "🦀 Welcome to Rust Web Server!",
        "info": "This is similar to Express.js but faster and safer",
        "author": "DevZAKRI"
    }))
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": "🟢 Server is running"
    })))
}

// GET /api/users?limit=10&offset=0
async fn get_users(
    Query(params): Query<QueryParams>,
    State(state): State<AppState>
) -> Json<serde_json::Value> {
    let db = state.lock().unwrap();
    let users: Vec<User> = db.values().cloned().collect();
    
    // Apply pagination (like in Node.js APIs)
    let limit = params.limit.unwrap_or(10) as usize;
    let offset = params.offset.unwrap_or(0) as usize;
    
    let paginated_users: Vec<User> = users
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();
    
    Json(serde_json::json!({
        "users": paginated_users,
        "total": db.len(),
        "limit": limit,
        "offset": offset
    }))
}

// GET /api/users/:id
async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>
) -> Result<Json<User>, StatusCode> {
    let db = state.lock().unwrap();
    
    match db.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// POST /api/users
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut db = state.lock().unwrap();
    
    // Generate new ID (in a real app, use UUID or database auto-increment)
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };
    
    db.insert(new_id, new_user.clone());
    
    Ok((StatusCode::CREATED, Json(new_user)))
}

// PUT /api/users/:id
async fn update_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<Json<User>, StatusCode> {
    let mut db = state.lock().unwrap();
    
    match db.get_mut(&id) {
        Some(user) => {
            user.name = payload.name;
            user.email = payload.email;
            user.age = payload.age;
            Ok(Json(user.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// DELETE /api/users/:id
async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut db = state.lock().unwrap();
    
    match db.remove(&id) {
        Some(_) => Ok(Json(serde_json::json!({
            "message": format!("User {} deleted successfully", id)
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn server_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "server": "Rust Web Server",
        "framework": "Axum",
        "version": "0.1.0",
        "rust_version": std::env!("RUSTC_VERSION"),
        "features": [
            "🔒 Memory Safety",
            "⚡ High Performance", 
            "🔧 Type Safety",
            "🚀 Zero-cost Abstractions",
            "🌐 Async/Await Support"
        ],
        "comparison_to_nodejs": {
            "performance": "2-3x faster than Node.js",
            "memory_usage": "Lower memory footprint",
            "type_safety": "Compile-time type checking",
            "concurrency": "Built-in async/await without event loop overhead"
        }
    }))
}
