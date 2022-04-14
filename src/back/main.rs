// For ORM : https://github.com/diesel-rs/diesel
// For Http Server: https://github.com/actix/actix-web
// For Http Client: https://github.com/seanmonstar/reqwest
// For Json: https://github.com/serde-rs
// For Config Files: https://github.com/mehcode/config-rs

// Example Rust + PostgreSQL + Diesel + actix-web: https://github.com/qoddiapps/rust_actix_diesel_connect

// https://github.com/balliegojr/todo-api

#[macro_use]
extern crate diesel;

#[path = "config/application.rs"]
mod config_app;

#[path = "config/database.rs"]
mod config_db;

#[path = "controller/router.rs"]
mod router;

#[path = "http-utils/br_encoding.rs"]
mod br_encoding;

use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = config_app::load_file_settings(
        "/Users/cybercastle/Documents/SoftwareProyects/iot-anlytcs/config/app-settings.yml",
    )
    .unwrap();

    if config_app::logger_initializer("debug".to_string()) {
        std::process::exit(1);
    }

    let address: String = settings.get("dbconnection.address").unwrap();
    let port: u16 = settings.get("dbconnection.port").unwrap();
    let database: String = settings.get("dbconnection.database").unwrap();
    let username: String = settings.get("dbconnection.username").unwrap();
    let password: String = settings.get("dbconnection.password").unwrap();
    let pg_pool_min: u32 = settings.get("dbconnection.poolMin").unwrap_or(5);
    let pg_pool_max: u32 = settings.get("dbconnection.poolMax").unwrap_or(10);

    let mut db_connection = config_db::DBConnection::new(
        address,
        port,
        database,
        username,
        password,
        pg_pool_min,
        pg_pool_max,
    );

    db_connection.open_connection();

    // TODO: quitar unwrap
    let pg_pool = db_connection.get_pool().unwrap();

    HttpServer::new(move || {
        App::new()
            // Setup DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pg_pool.clone()))
            .service(web::scope("/api").configure(router::route))
            .service(
                web::scope("")
                    .wrap(br_encoding::BrotliEncodingHeader)
                    .service(Files::new("/", "./dist").index_file("index.html")),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
