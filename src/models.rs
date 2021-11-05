use super::schema::{cannabis, inventories, products};

use diesel::pg::PgConnection;
use diesel::sql_types::{Float, Integer, VarChar};
use diesel::{sql_query, RunQueryDsl};
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
#[derive(Debug, DbEnum, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "cannabis"]
pub struct NewCannabis {
    product_id: i32,
    family: Family,
    thc: f32,
    cbd: f32,
    total_cannabinoids: f32,
}

impl NewCannabis {
    pub fn new(
        product_id: i32,
        family: Family,
        thc: f32,
        cbd: f32,
        total_cannabinoids: f32,
    ) -> Self {
        NewCannabis {
            product_id,
            family,
            thc,
            cbd,
            total_cannabinoids,
        }
    }
}

#[derive(Debug, Serialize, Queryable, QueryableByName)]
#[table_name = "cannabis"]
pub struct Cannabis {
    id: i32,
    product_id: i32,
    family: Family,
    thc: f32,
    cbd: f32,
    total_cannabinoids: f32,
}

impl Cannabis {
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_product_id(&self) -> &i32 {
        &self.product_id
    }

    pub fn get_family(&self) -> &Family {
        &self.family
    }

    pub fn get_thc(&self) -> &f32 {
        &self.thc
    }

    pub fn get_cbd(&self) -> &f32 {
        &self.cbd
    }

    pub fn get_total_cannabinoids(&self) -> &f32 {
        &self.total_cannabinoids
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "inventories"]
pub struct NewInventory {
    product_id: i32,
    stock: i32,
    price: f32,
    net_weight: f32,
}

impl NewInventory {
    pub fn new(product_id: i32, stock: i32, price: f32, net_weight: f32) -> Self {
        NewInventory {
            product_id,
            stock,
            price,
            net_weight,
        }
    }
}

#[derive(Debug, Serialize, Queryable, QueryableByName)]
#[table_name = "inventories"]
pub struct Inventory {
    id: i32,
    product_id: i32,
    stock: i32,
    price: f32,
    net_weight: f32,
}

#[derive(Debug, Serialize, Queryable, QueryableByName)]
pub struct InventoryResponse {
    #[sql_type = "Integer"]
    id: i32,

    #[sql_type = "Integer"]
    product_id: i32,

    #[sql_type = "VarChar"]
    name: String,

    #[sql_type = "CategoryMapping"]
    category: Category,

    #[sql_type = "Integer"]
    stock: i32,

    #[sql_type = "Float"]
    price: f32,

    #[sql_type = "Float"]
    net_weight: f32,
}

impl Inventory {
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn with_product_id(
        conn: &PgConnection,
        prod_id: &i32,
    ) -> Result<Vec<InventoryResponse>, diesel::result::Error> {
        let _stmt = "SELECT
                      i.id, i.product_id, p.name, p.category, i.stock, i.price, i.net_weight
                    FROM inventories i INNER JOIN products p ON i.product_id = p.id
                    WHERE p.id = $1";
        sql_query(_stmt)
            .bind::<Integer, _>(prod_id)
            .get_results(conn)
    }
}
