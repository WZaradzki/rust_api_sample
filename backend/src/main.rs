use actix_web::{web, App, HttpServer};
use backend::routes;
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/public").configure(routes::public::public_routes))
            .service(web::scope("/admin").configure(routes::admin::admin_routes))
            .service(web::scope("/user").configure(routes::user::user_routes))
    })
    .bind(("127.0.0.1", 2122))?
    .run()
    .await
}
