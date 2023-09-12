use std::usize;

use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Default, Serialize, Deserialize)]
pub struct Role {
    model: Model,
    name: String,
    remark: String,
    group_name: String,
}

impl Role {
    pub fn set_id(&mut self, id: usize) {
        self.model.id = id
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct RoleGroup {
    model: Model,
    name: String,
    remark: String,
    roles: Vec<Role>,
}

impl RoleGroup {
    pub fn set_id(&mut self, id: usize) {
        self.model.id = id
    }
}
