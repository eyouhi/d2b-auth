use serde::Serialize;

use super::Model;

#[derive(Default, Serialize)]
pub struct DeptOrder {
    dept_id: usize,
    order: i32,
}

#[derive(Default, Serialize)]
pub struct LeaderInDept {
    dept_id: usize,
    leader: String,
}

#[derive(Default, Serialize)]
pub struct Dept {
    model: Model,
    ding_id: usize,
    name: String,
    order: i32,
    sub: Vec<Dept>,
}

impl Dept {
    pub fn set_id(&mut self, id: usize) {
        self.model.id = id
    }
}
