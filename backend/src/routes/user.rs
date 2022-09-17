use actix_web::{web};
use crate::handlers::user::users;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(users::get_users))
        );
}