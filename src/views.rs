use super::models::*;
use super::{Creatable, DbPool, Field, Readable};

use actix_web::{get, post, web, HttpResponse, Result};

use handlebars::Handlebars;

use serde_json::json;

#[get("/products/new/")]
pub async fn new_product_form(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "fields": Category::fields() });
    let body = hb.render("new_product_form", &data).unwrap();
    HttpResponse::Ok().body(&body)
}

#[post("/products/new/")]
pub async fn post_new_product_form(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<NewProduct>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("Couldn't get connection from pool.");
    web::block(move || form.into_inner().create(&conn))
        .await
        .map(|prod| {
            let data = json!({ "new_product": prod, "fields": Category::fields() });
            let body = hb.render("new_product_form", &data).unwrap();
            HttpResponse::Ok().body(&body)
        })
        .map_err(|e| {
            let data = json!({"error": e.to_string(), "fields": Category::fields()});
            let body = hb.render("new_product_form", &data).unwrap();
            HttpResponse::InternalServerError().body(&body)
        })
}

#[get("/products/cannabis/new/")]
pub async fn new_cannabis_form(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mut errors = Vec::<String>::new();
    let conn = pool.get().expect("Couldn't get connection from pool.");
    web::block(move || Product::all(&conn))
        .await
        .map(|prods| {
            let data = json!({ "products": prods, "fields": Family::fields() });
            let body = hb.render("new_cannabis_form", &data).unwrap();
            HttpResponse::Ok().body(&body)
        })
        .map_err(|e| {
            errors.push(e.to_string());
            let data = json!({ "errors": errors, "fields": Family::fields() });
            let body = hb.render("new_cannabis_form", &data).unwrap();
            HttpResponse::InternalServerError().body(&body)
        })
}
