use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct SensorInfo {
    pub id: i64,
    pub sensor_name: String,
    pub sensor_type: i32,
    pub register_date: NaiveDate,
    pub is_enabled: bool,
    pub sensor_description: String,
    pub measurement_unit: Option<i32>,
    pub random_serial: Option<f64>,
    pub last_check_status: Option<i32>,
    pub last_check_status_date: Option<NaiveDateTime>,
}
