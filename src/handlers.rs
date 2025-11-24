use actix_web::{web, HttpResponse, Responder};
use crate::models::{Todo, TodoInput, UpdateInput, DB};

// Create Todo
pub async fn create_todo(
    db: web::Data<DB>,
    payload: web::Json<TodoInput>,
) -> impl Responder {
    let todo = Todo::new(payload.title.clone());
    db.lock().unwrap().insert(todo.id.clone(), todo.clone());

    HttpResponse::Ok().json(todo)
}

// Get all Todos
pub async fn get_todos(db: web::Data<DB>) -> impl Responder {
    let todos = db.lock().unwrap()
        .values()
        .cloned()
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(todos)
}

// Update Todo
pub async fn update_todo(
    db: web::Data<DB>,
    id: web::Path<String>,
    payload: web::Json<UpdateInput>,
) -> impl Responder {
    let mut map = db.lock().unwrap();

    if let Some(todo) = map.get_mut(id.as_str()) {
        if let Some(t) = &payload.title {
            todo.title = t.clone();
        }
        if let Some(c) = payload.completed {
            todo.completed = c;
        }
        return HttpResponse::Ok().json(todo.clone());
    }

    HttpResponse::NotFound().body("Todo not found")
}

// Delete Todo
pub async fn delete_todo(
    db: web::Data<DB>,
    id: web::Path<String>,
) -> impl Responder {
    let removed = db.lock().unwrap().remove(id.as_str()).is_some();
    HttpResponse::Ok().json(removed)
}
