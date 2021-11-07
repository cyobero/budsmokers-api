#[macro_use]
extern crate diesel;

use self::models::*;
use self::schema::users::dsl::users;

mod models;
mod schema;

use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Connection, RunQueryDsl};

use serde::{Deserialize, Serialize};

pub trait Identifiable<'id> {
    type Id: Deserialize<'id> + Serialize;

    fn get_id(&self) -> &Self::Id;
}

impl<'id> Identifiable<'id> for User {
    type Id = i32;

    fn get_id(&self) -> &i32 {
        self._get_id()
    }
}

pub trait Creatable<Db = Pg, Conn = PgConnection, Er = Error>
where
    Db: Backend,
    Conn: Connection,
{
    type Output;

    fn create(&self, conn: &Conn) -> Result<Self::Output, Er>;
}

impl Creatable for NewUser {
    type Output = User;

    fn create(&self, conn: &PgConnection) -> Result<User, Error> {
        diesel::insert_into(users).values(self).get_result(conn)
    }
}
