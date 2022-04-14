mod sensor_info_controller;

use actix_web::web::ServiceConfig;

use self::sensor_info_controller::get_sensors;

pub fn route(config: &mut ServiceConfig) {
    config.service(get_sensors);
}
