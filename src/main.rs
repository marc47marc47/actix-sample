use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define a user structure
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// Shared state to store users
type UserData = Arc<Mutex<HashMap<u32, String>>>;

// Handler to get user by id
async fn get_user(data: web::Data<UserData>, user_id: web::Path<u32>) -> impl Responder {
    let user_map = data.lock().unwrap();
    let user_id = user_id.into_inner();
    if let Some(name) = user_map.get(&user_id) {
        HttpResponse::Ok().json(User { id: user_id, name: name.clone() })
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

// Handler to add a new user
async fn add_user(data: web::Data<UserData>, user: web::Json<User>) -> impl Responder {
    let mut user_map = data.lock().unwrap();
    user_map.insert(user.id, user.name.clone());
    HttpResponse::Created().body(format!("User {} added successfully", user.name))
}

// Main function to set up the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Shared state for user data
    let user_data: UserData = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_data.clone()))
            .route("/user/{id}", web::get().to(get_user))
            .route("/user", web::post().to(add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

