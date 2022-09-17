use crate::handlers::admin::users;
use actix_web::web;

pub fn admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(users::get_users)));
}
