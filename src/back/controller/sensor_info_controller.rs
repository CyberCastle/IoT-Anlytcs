#[path = "../service/sensor_info_service.rs"]
mod sensor_info_service;

use self::sensor_info_service::fetch_all_sensors;
use crate::config_db::PgPool;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/sensors")]
pub async fn get_sensors(pool: Data<PgPool>) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Couldn't get a connection"),
    };

    match web::block(move || fetch_all_sensors(&conn)).await {
        Ok(sensors) => HttpResponse::Ok().json(sensors.unwrap()),
        Err(err) => HttpResponse::InternalServerError().body(format!("Some error ocurred {}", err)),
    }
}
