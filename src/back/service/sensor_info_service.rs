#[path = "../domain/mod.rs"]
mod domain;

use diesel::{prelude::*, result::Error};

use self::domain::{entities::SensorInfo, schema::sensor_info};

pub fn fetch_all_sensors(conn: &PgConnection) -> Result<Vec<SensorInfo>, Error> {
    sensor_info::table
        //.filter(is_enabled.eq(true))
        //.limit(1)
        //.offset(1)
        .select(sensor_info::all_columns)
        .load::<SensorInfo>(conn)
}
