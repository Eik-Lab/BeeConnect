use crate::schema::{hive_infos, measurements};
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::{pg::PgConnection, Associations, Identifiable, Queryable};
use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(
    Identifiable, Insertable, Queryable, Deserialize, Serialize, Debug, Clone, AsChangeset,
)]
#[diesel(primary_key(hive_id))]
pub struct HiveInfo {
    pub hive_id: i32,
    pub info: String,
    pub longitude: f32,
    pub altitude: f32,
}

#[derive(
    Identifiable, Insertable, Queryable, Deserialize, Serialize, Debug, Clone, AsChangeset,
)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(hive_id))]
pub struct Measurement {
    pub id: uuid::Uuid,
    pub hive_id: i32,
    pub layer: i32,
    pub time: DateTime<Utc>,
    pub weight: f32,
    pub temp: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMeasurement {
    pub hive_id: i32,
    pub layer: i32,
    pub time: DateTime<Utc>,
    pub weight: f32,
    pub temp: Vec<f32>,
}

impl From<NewMeasurement> for Measurement {
    fn from(new_measurement: NewMeasurement) -> Self {
        Self {
            id: Uuid::new_v4(),
            hive_id: new_measurement.hive_id,
            layer: new_measurement.layer,
            time: new_measurement.time,
            weight: new_measurement.weight,
            temp: new_measurement.temp,
        }
    }
}
