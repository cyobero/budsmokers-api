use super::*;

use diesel::pg::PgConnection;
use diesel::Connection;

use dotenv::dotenv;

use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).expect("Could not establish connection.")
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::{establish_connection, Creatable, Deletable};

    #[test]
    fn user_created_and_deleted() {
        let conn = establish_connection();
        let new = NewUser::new("testuser3000", "password123").create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());
    }
}
