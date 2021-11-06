use products::handlers::*;
use products::views::*;

use actix_web::{web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use handlebars::Handlebars;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up db pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    // set up template rendering
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8888...\n");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(post_product)
            .service(get_product_id)
            .service(post_cannabis)
            .service(get_cannabis_id)
            .service(post_inventory)
            .service(get_product_inventory)
            .service(new_product_form)
            .service(post_new_product_form)
            .service(new_cannabis_form)
            .service(post_new_cannabis_form)
            .service(new_inventory_form)
    })
    .bind("192.168.0.6:8888")?
    .run()
    .await
}
