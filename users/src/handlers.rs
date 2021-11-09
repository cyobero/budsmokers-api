use super::{Cleanable, Creatable, Hashable, Verifiable};
use super::{DbPool, NewUserInput};

use actix_web::{delete, get, post, web, HttpResponse, Responder};

use serde_json::json;

#[post("/users/register")]
pub async fn register_handler(
    pool: web::Data<DbPool>,
    form: web::Form<NewUserInput>,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection from pool.");
    let cleaned = form.into_inner().clean();

    match cleaned {
        Err(e) => Ok(HttpResponse::InternalServerError()
            .json(json!({"status": 500, "message": e.to_string()}))),

        Ok(usr) => web::block(move || {
            usr.hash_password()
                .verify(&conn)
                .and_then(|nu| nu.create(&conn))
        })
        .await
        .map(|usr| HttpResponse::Ok().json(json!({"status": 200, "data": usr})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status": 500, "message": e.to_string()}))
        }),
    }
}
