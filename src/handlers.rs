use super::models::*;
use super::{Creatable, DbPool, Readable};

use actix_web::{get, post, web, HttpResponse, Result};

#[post("/products")]
pub async fn post_product(
    pool: web::Data<DbPool>,
    form: web::Form<NewProduct>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || form.into_inner().create(&conn))
        .await
        .map(|prod| HttpResponse::Ok().json(prod))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/products/{id}")]
pub async fn get_product_id(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Could not get connection from pool.");
    web::block(move || Product::with_id(&conn, &path.into_inner().0))
        .await
        .map(|prod| HttpResponse::Ok().json(prod))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
