use crate::schema::post_uuids;
use diesel::prelude::*;
use diesel::{pg::PgConnection, Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(
    Identifiable, Insertable, Selectable, Queryable, Deserialize, Serialize, Debug, PartialEq,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = post_uuids)]
pub struct PostUuid {
    pub id: uuid::Uuid,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMes {
    pub msg: String,
}

impl From<NewMes> for PostUuid {
    fn from(new_mes: NewMes) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            msg: new_mes.msg,
        }
    }
}

impl PostUuid {
    pub fn get_all(connection: &mut PgConnection) -> QueryResult<Vec<PostUuid>> {
        post_uuids::dsl::post_uuids
            .select((post_uuids::id, post_uuids::msg))
            .load::<PostUuid>(connection)
    }
}
