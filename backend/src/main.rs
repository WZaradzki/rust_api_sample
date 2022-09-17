use actix_web::{web, App, HttpServer};

mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/public").configure(routes::public::public_routes))
            .service(web::scope("/admin").configure(routes::admin::admin_routes))
            .service(web::scope("/user").configure(routes::user::user_routes))
    })
    .bind(("127.0.0.1", 2122))?
    .run()
    .await
}
