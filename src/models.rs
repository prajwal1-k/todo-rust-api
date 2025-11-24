use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct TodoInput {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateInput {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

pub type DB = Arc<Mutex<HashMap<String, Todo>>>;

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            completed: false,
        }
    }
}
