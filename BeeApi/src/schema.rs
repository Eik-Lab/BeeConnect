use diesel::prelude::*;

#[derive(Queryable)]
pub struct Measurement {
    pub id: i32,
    pub temp: i32,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct NewMeasurment {
    pub temp: i32,
}
