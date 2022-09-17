use actix_web::{HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("USER USERS LIST!")
}
