#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::*;

    #[test]
    fn product_created_and_deleted() {
        let conn = establish_connection().unwrap();
        let new = NewProduct::new("OG Kush", Category::Flower).create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());
    }
}
