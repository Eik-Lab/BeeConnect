use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;

use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Insertable, Queryable, Deserialize, Serialize, Debug, Clone, AsChangeset,
)]
#[primary_key(id)]
pub struct Hello {
    pub id: i32,
    pub temp1x1x1: f32,
}
