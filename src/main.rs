use crate::controller::{create_user, delete_user, get_user_by_id, list_users, update_user};
use crate::user_service::UserService;
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

mod controller;
mod model;
mod user_service;

#[tokio::main]
async fn main() {
    println!("Starting Service..!");

    let service = UserService::new().await.unwrap();

    let app = Router::new()
        .route("/users", get(list_users))
        .route("/user/:id", get(get_user_by_id))
        .route("/user", post(create_user))
        .route("/user/:id", put(update_user))
        .route("/user/:id", delete(delete_user))
        .layer(Extension(service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening..!");
    axum::serve(listener, app).await.unwrap();
}
