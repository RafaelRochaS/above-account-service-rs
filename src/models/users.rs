use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub email: String,
    pub age: u16,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub email: String,
    pub age: u16,
}
