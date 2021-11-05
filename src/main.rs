use products::handlers::*;

use actix_web::{App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up db pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    println!("Listening on port 8888...\n");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(post_product)
            .service(get_product_id)
            .service(post_cannabis)
            .service(get_cannabis_id)
    })
    .bind("192.168.0.6:8888")?
    .run()
    .await
}
