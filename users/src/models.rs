use super::schema::users;
use super::VerificationError;

use diesel::pg::PgConnection;
use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, Error};
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

use sha_crypt::{sha512_simple, Sha512Params};

#[derive(Debug, Deserialize, Serialize, Insertable)]
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

    pub fn _verify_username(self, conn: &PgConnection) -> Result<NewUser, Error> {
        let usrs: Vec<User> = users::table
            .filter(users::username.eq(self.get_username()))
            .get_results(conn)
            .expect("Couldn't verify username.");

        match usrs.is_empty() {
            true => Ok(self),
            false => Err(Error::NotFound),
        }
    }

    pub fn _hash_password(mut self) -> Self {
        let params = Sha512Params::new(10_000).expect("Random error.");
        self.password = sha512_simple(&self.password, &params).unwrap();
        self
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

    pub fn _get_password(&self) -> &String {
        &self.password
    }
}
