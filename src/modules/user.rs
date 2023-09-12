use super::{
    dept::{DeptOrder, LeaderInDept},
    role::Role,
    Model,
};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct User {
    model: Model,
    ding_id: String,
    manager_userid: String,
    union_id: String,
    name: String,
    avatar: String,
    state_code: String,
    mobile: String,
    hide_mobile: bool,
    telephone: String,
    job_number: String,
    title: String,
    email: String,
    work_place: String,
    remark: String,
    exclusive_account: bool,
    org_email: String,
    dept_id_list: Vec<usize>,
    dept_order_list: Vec<DeptOrder>,
    extension: String,
    hired_date: i32,
    active: bool,
    real_authed: bool,
    senior: bool,
    admin: bool,
    boss: bool,
    leader_in_dept: Vec<LeaderInDept>,
    role_list: Vec<Role>,
}

impl User {
    pub fn set_id(&mut self, id: usize) {
        self.model.id = id;
    }
}

#[derive(Default, Serialize)]
pub struct UserGroup {
    model: Model,
    name: String,
    remark: String,
    user_list: Vec<User>,
}

impl UserGroup {
    pub fn set_id(&mut self, id: usize) {
        self.model.id = id;
    }
}
