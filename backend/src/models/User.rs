use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}