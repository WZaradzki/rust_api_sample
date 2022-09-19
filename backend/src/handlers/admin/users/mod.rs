use crate::{
    create_connection,
    models::User::{self, UserModel},
    schema::users::dsl::*,
};
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_users() -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let connection = &mut create_connection();
        users.load::<User::UserModel>(connection);
    })
    .await?;

    if user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {user_uid}"));
        Ok(res)
    }
}
