use super::schema::users;

use diesel::Insertable;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}

impl NewUser {
    pub fn new(username: &str, password: &str) -> Self {
        NewUser {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }
}

#[derive(Deserialize, Serialize, Queryable, QueryableByName)]
#[table_name = "users"]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

impl User {
    pub fn _get_username(&self) -> &String {
        &self.username
    }

    pub fn _get_id(&self) -> &i32 {
        &self.id
    }
}
