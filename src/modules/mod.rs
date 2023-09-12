pub mod user;

pub mod role;

pub mod dept;

pub mod auth;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Model {
    id: usize,
    created_at: String,
    updated_at: String,
}
