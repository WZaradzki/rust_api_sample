use actix_web::{HttpResponse, Responder};

pub async fn get_courses() -> impl Responder {
    HttpResponse::Ok().body("COURSES")
}
