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
    use super::FormError;
    use super::{establish_connection, Cleanable, Creatable, Deletable, NewUserInput};
    use diesel::pg::PgConnection;

    #[test]
    fn user_created_and_deleted() {
        let conn = establish_connection();
        let new = NewUser::new("testuser3000", "password123").create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());
    }

    #[test]
    fn empty_username_raises_error() {
        let conn = establish_connection();
        let input = NewUserInput {
            username: "".to_owned(),
            password1: "password123".to_owned(),
            password2: "password123".to_owned(),
        };
        assert!(input.clean_username().is_err());
        let input = NewUserInput {
            username: "abc".to_owned(),
            password1: "password1".to_owned(),
            password2: "password1".to_owned(),
        };
        assert!(input.clean_username().is_err());
        let input = NewUserInput {
            username: "cyobero".to_owned(),
            password1: "password123".to_owned(),
            password2: "password123".to_owned(),
        };
        assert!(input.clean_username().is_ok());
    }

    #[test]
    fn empty_password_fails() {
        let input = NewUserInput {
            username: "cyobero".to_owned(),
            password1: "".to_owned(),
            password2: "donesntmatter".to_owned(),
        };
        assert!(input.clean_password().is_err());
    }

    #[test]
    fn password_mismatch_fails() {
        let input = NewUserInput {
            username: "cyobero".to_owned(),
            password1: "password123".to_owned(),
            password2: "password419".to_owned(),
        };
        assert!(input.clean_password().is_err());
    }

    #[test]
    fn clean_password_succeeds() {
        let input = NewUserInput {
            username: "cyobero".to_owned(),
            password1: "password123".to_owned(),
            password2: "password123".to_owned(),
        };
        assert!(input.clean_password().is_ok());
    }

    #[test]
    fn new_user_input_is_cleaned() {
        let input = NewUserInput {
            username: "cyobero".to_owned(),
            password1: "password123".to_owned(),
            password2: "password123".to_owned(),
        };
        let _new = input.clean();
        assert!(&_new.is_ok());
        assert_eq!(_new.unwrap().get_username(), "cyobero");
    }

    #[test]
    fn user_already_exists_fails() {
        let conn = establish_connection();
        let _new = NewUser::new("testuser1", "password69")._verify_username(&conn);
        assert!(_new.is_err());
    }

    #[test]
    fn username_is_available() {
        let conn = establish_connection();
        let _new = NewUser::new("testuser3000", "mypassword")._verify_username(&conn);
        assert!(_new.is_ok());
    }
}
