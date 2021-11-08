#[macro_use]
extern crate diesel;

use self::models::*;
use self::schema::users::dsl::users;

mod models;
mod schema;
mod tests;

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

pub trait Deletable<Id = i32, Conn = PgConnection, Er = Error>
where
    Id: PartialEq,
    Conn: Connection,
{
    type Output;
    fn delete(&self, conn: &Conn) -> Result<Self::Output, Er>;
}

pub trait Readable<Id = i32, Conn = PgConnection, Er = Error>
where
    Id: PartialEq,
    Conn: Connection,
{
    type Output;

    fn all(conn: &PgConnection) -> Result<Vec<Self::Output>, Er>;
    fn with_id(conn: &PgConnection, _id: &Id) -> Result<Self::Output, Er>;
}

pub trait Creatable<Conn = PgConnection, Er = Error>
where
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

impl Readable for User {
    type Output = User;

    fn all(conn: &PgConnection) -> Result<Vec<User>, Error> {
        users.load(conn)
    }

    fn with_id(conn: &PgConnection, _id: &i32) -> Result<User, Error> {
        users.find(_id).get_result(conn)
    }
}

impl Deletable for User {
    type Output = User;

    fn delete(&self, conn: &PgConnection) -> Result<User, Error> {
        diesel::delete(users.find(self._get_id())).get_result(conn)
    }
}
