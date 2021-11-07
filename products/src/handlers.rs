use super::models::*;
use super::{Creatable, DbPool, Readable};

use actix_web::{get, post, web, HttpResponse, Result};

use serde_json::json;

#[post("/products")]
pub async fn post_product(
    pool: web::Data<DbPool>,
    form: web::Form<NewProduct>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || form.into_inner().create(&conn))
        .await
        .map(|prod| HttpResponse::Ok().json(json!({"status": 200, "data": prod})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[get("/products")]
pub async fn get_products(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || Product::all(&conn))
        .await
        .map(|prods| HttpResponse::Ok().json(json!({"status": 200, "data": prods})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[get("/products/{id}")]
pub async fn get_product_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || Product::with_id(&conn, &path.into_inner()))
        .await
        .map(|prod| HttpResponse::Ok().json(json!({"status": 200, "data": prod})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[post("/products/cannabis")]
pub async fn post_cannabis(
    pool: web::Data<DbPool>,
    form: web::Form<NewCannabis>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || form.into_inner().create(&conn))
        .await
        .map(|cnbs| HttpResponse::Ok().json(json!({"status": 200, "data": cnbs})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[get("/products/cannabis/{id}")]
pub async fn get_cannabis_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || Cannabis::with_id(&conn, &path.into_inner()))
        .await
        .map(|cnbs| HttpResponse::Ok().json(json!({"status": 200, "data": cnbs})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[post("/inventories")]
pub async fn post_inventory(
    pool: web::Data<DbPool>,
    form: web::Form<NewInventory>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || form.into_inner().create(&conn))
        .await
        .map(|inv| HttpResponse::Ok().json(json!({"status": 200, "data": inv})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[get("/products/{id}/inventory")]
pub async fn get_product_inventory(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || Inventory::with_product_id(&conn, &path.into_inner()))
        .await
        .map(|inv| HttpResponse::Ok().json(json!({"status": 200, "data": inv})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}

#[get("/inventories")]
pub async fn get_inventories(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || InventoryResponse::all(&conn))
        .await
        .map(|inv| HttpResponse::Ok().json(json!({"status": 200, "data": inv})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 404, "message": e.to_string()}))
        })
}
