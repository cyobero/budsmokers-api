#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::*;

    #[test]
    fn product_created_and_deleted() {
        let conn = establish_connection().unwrap();
        let new = NewProduct::new("OG Kush #69", Category::Flower).create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());
    }

    #[test]
    fn cannabis_created_and_deleted() {
        let conn = establish_connection().unwrap();
        let _prod = NewProduct::new("Reggie Kush #5", Category::Flower)
            .create(&conn)
            .unwrap();
        let new = NewCannabis::new(*_prod.get_id(), Family::Indica, 22.5, 0.5, 23.1).create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());

        let _ = _prod.delete(&conn);
    }

    #[test]
    fn inventory_created_and_deleted() {
        let conn = establish_connection().unwrap();
        let _prod = NewProduct::new("Reggie Kush #15", Category::Flower)
            .create(&conn)
            .unwrap();

        let new = NewInventory::new(*_prod.get_id(), 10, 15.0, 1.0).create(&conn);

        assert!(new.is_ok());

        let deleted = new.unwrap().delete(&conn);

        assert!(deleted.is_ok());

        let _ = _prod.delete(&conn);
    }
}
