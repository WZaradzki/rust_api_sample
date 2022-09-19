use crate::{
    models::User::{self},
};
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

// pub async fn get_users() -> Result<HttpResponse, Error> {
//     let user = web::block(move || {
//         let connection = &mut create_connection();
//         users.load::<User::UserModel>(connection);
//     })
//     .await?;

//     if user {
//         Ok(HttpResponse::Ok().json(user))
//     } else {
//         let res = HttpResponse::NotFound().body(format!("No user found with uid: {user_uid}"));
//         Ok(res)
//     }
// }
pub fn action_get_users(
    conn: &mut MysqlConnection,
) -> Result<Vec<User::User>, DbError> {
    use crate::schema::users::dsl::*;

    let users_query = users.load::<User::User>(conn).expect("FAIL");

    Ok(users_query)
}

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || {
        let mut conn = pool.get()?;
        action_get_users(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !results.is_empty() {
        Ok(HttpResponse::Ok().json(results))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid"));
        Ok(res)
    }
}
