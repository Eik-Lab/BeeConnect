use crate::schema::posts;

use chrono::offset::Utc;
use chrono::DateTime;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::{pg::PgConnection, Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Clone, Queryable, Insertable, Deserialize, Serialize)]
pub struct Post {
    pub id: uuid::Uuid,
    pub msg: String,
}

#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct Measurement {
    pub post: Post,
}
