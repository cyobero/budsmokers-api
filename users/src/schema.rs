table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}
