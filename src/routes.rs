// In this here we give the routes to the todos
use actix_web::web;
use crate::handlers::{create_todo, get_todos, update_todo, delete_todo};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/todos", web::post().to(create_todo))
        .route("/todos", web::get().to(get_todos))
        .route("/todos/{id}", web::put().to(update_todo))
        .route("/todos/{id}", web::delete().to(delete_todo));
}
