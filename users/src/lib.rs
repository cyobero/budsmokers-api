#[macro_use]
extern crate diesel;

use self::models::*;
use self::schema::users::dsl::users;

pub mod handlers;
mod models;
mod schema;
mod tests;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{Connection, RunQueryDsl};

use serde::{Deserialize, Serialize};

use sha_crypt::{sha512_simple, Sha512Params};

use std::string::ToString;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Serialize)]
pub enum FormError {
    EmptyField,
    FieldTooShort,
    PasswordMismatch,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VerificationError {
    AlreadyExists,
    NotExists,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserInput {
    pub username: String,
    pub password1: String,
    pub password2: String,
}

impl NewUserInput {
    pub fn clean_username(self) -> Result<NewUserInput, FormError> {
        match &self.username.len() {
            0 => Err(FormError::EmptyField),
            1..=3 => Err(FormError::FieldTooShort),
            _ => Ok(self),
        }
    }

    pub fn clean_password(self) -> Result<NewUserInput, FormError> {
        match &self.password1.len() {
            0 => Err(FormError::EmptyField),
            1..=7 => Err(FormError::FieldTooShort),
            _ => match &self.password1 == &self.password2 {
                true => Ok(self),
                false => Err(FormError::PasswordMismatch),
            },
        }
    }
}

pub trait Hashable<T, U = Self> {
    fn hash_password(self) -> U;
}

pub trait Verifiable<Conn = PgConnection>
where
    Conn: Connection,
{
    type Output;
    type Error;

    fn verify(self, conn: &Conn) -> Result<Self::Output, Self::Error>;
}

pub trait Cleanable {
    type Output;
    type Error;

    fn clean(self) -> Result<Self::Output, Self::Error>;
}

impl Cleanable for NewUserInput {
    type Output = NewUser;
    type Error = FormError;

    fn clean(self) -> Result<NewUser, FormError> {
        let _clean = self.clean_username().and_then(|s| s.clean_password())?;
        let _new = NewUser::new(&_clean.username, &_clean.password1);
        Ok(_new)
    }
}

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

impl Verifiable for NewUser {
    type Output = NewUser;
    type Error = diesel::result::Error;

    fn verify(self, conn: &PgConnection) -> Result<NewUser, Error> {
        self._verify_username(&conn)
    }
}

impl ToString for FormError {
    fn to_string(&self) -> String {
        format!("FormError: {:?}", &self)
    }
}

impl Hashable<String, NewUserInput> for NewUserInput {
    fn hash_password(mut self) -> Self {
        let params = Sha512Params::new(10_000).expect("Random error.");
        self.password1 = sha512_simple(&self.password1, &params).unwrap();
        self
    }
}

impl Hashable<String, Self> for NewUser {
    fn hash_password(self) -> NewUser {
        self._hash_password()
    }
}
