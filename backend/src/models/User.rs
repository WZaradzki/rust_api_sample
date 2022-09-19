use diesel::Queryable;

#[derive(Queryable)]
pub struct UserModel {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}