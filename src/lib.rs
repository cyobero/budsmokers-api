#[macro_use]
extern crate diesel;

use self::models::*;
use self::schema::cannabis::dsl::cannabis;
use self::schema::products::dsl::*;

use diesel::backend::Backend;
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{self, ConnectionManager};
use diesel::{
    result::Error, Connection, ConnectionError, Insertable, QueryDsl, Queryable, RunQueryDsl,
};

use std::env;

pub mod handlers;
mod models;
mod schema;
mod tests;

pub mod exports {
    pub use super::models::CategoryMapping as Category;
    pub use super::models::FamilyMapping as Family;
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

pub trait Creatable<Db = Pg, Conn = PgConnection, Er = Error>
where
    Db: Backend,
    Conn: Connection,
{
    type Object;

    fn create(&self, conn: &Conn) -> Result<Self::Object, Er>;
}

pub trait Readable<Id = i32, Obj = Self, Db = Pg, Conn = PgConnection, Er = Error>
where
    Id: Eq,
    Db: Backend,
    Conn: Connection,
{
    fn all(conn: &Conn) -> Result<Vec<Obj>, Er>;
    fn with_id(conn: &Conn, _id: &i32) -> Result<Obj, Er>;
}

pub trait Deletable<Id = i32, Obj = Self, Db = Pg, Conn = PgConnection, Er = Error>
where
    Id: Eq,
    Db: Backend,
    Conn: Connection,
{
    fn delete(&self, conn: &Conn) -> Result<Obj, Er>;
}

impl Creatable for NewProduct {
    type Object = Product;

    fn create(&self, conn: &PgConnection) -> Result<Product, Error> {
        diesel::insert_into(products).values(self).get_result(conn)
    }
}

impl Creatable for NewCannabis {
    type Object = Cannabis;

    fn create(&self, conn: &PgConnection) -> Result<Cannabis, Error> {
        diesel::insert_into(cannabis).values(self).get_result(conn)
    }
}

impl Readable for Product {
    fn all(conn: &PgConnection) -> Result<Vec<Product>, Error> {
        products.load(conn)
    }

    fn with_id(conn: &PgConnection, _id: &i32) -> Result<Product, Error> {
        products.find(_id).get_result(conn)
    }
}

impl Readable for Cannabis {
    fn all(conn: &PgConnection) -> Result<Vec<Cannabis>, Error> {
        cannabis.load(conn)
    }

    fn with_id(conn: &PgConnection, _id: &i32) -> Result<Cannabis, Error> {
        cannabis.find(_id).get_result(conn)
    }
}

impl Deletable for Product {
    fn delete(&self, conn: &PgConnection) -> Result<Product, Error> {
        diesel::delete(products.find(self.get_id())).get_result(conn)
    }
}

impl Deletable for Cannabis {
    fn delete(&self, conn: &PgConnection) -> Result<Cannabis, Error> {
        diesel::delete(cannabis.find(self.get_id())).get_result(conn)
    }
}
