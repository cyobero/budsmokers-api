use super::schema::products;

use diesel_derive_enum::DbEnum;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, DbEnum)]
pub enum Category {
    Flower,
    PreRoll,
    Edible,
    Cartridge,
    Extract,
    Accessory,
    Other,
}

#[derive(Debug, DbEnum)]
pub enum Family {
    Indica,
    Sativa,
    Hybrid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub category: Category,
}

impl NewProduct {
    pub fn new(name: &str, category: Category) -> Self {
        NewProduct {
            name: name.to_owned(),
            category,
        }
    }
}

#[derive(Serialize, Queryable, QueryableByName)]
#[table_name = "products"]
pub struct Product {
    id: i32,
    name: String,
    category: Category,
}

impl Product {
    pub fn new(id: i32, name: String, category: Category) -> Self {
        Product { id, name, category }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_category(&self) -> &Category {
        &self.category
    }
}
