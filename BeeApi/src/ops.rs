use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::Measurement;

fn add_mes(temp: &i32) {
    let new_mes = NewMeasurment {
        temp: temp.to_owned(),
    };

    diesel::insert_into(measurements::table).values(&new_mes)
}

fn find_mes_by_id(id: &i32) -> Option<Measurement> {
    measurements::table.filter(measurements::id.eq(id))
}
