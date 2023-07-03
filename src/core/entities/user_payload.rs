use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserPayload {
    id: i32,
    name: String,
    surname: String,
}

impl UserPayload {
    pub fn new(id: i32, name: String, surname: String) -> Self {
        Self { id, name, surname }
    }
}
