use concat_strs::concat_strs; // https://github.com/9999years/concat_strs
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DBConnection {
    address: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    pg_pool_min: u32,
    pg_pool_max: u32,
    pool: Option<PgPool>,
}

impl DBConnection {
    pub fn new(
        address: String,
        port: u16,
        database: String,
        username: String,
        password: String,
        pg_pool_min: u32,
        pg_pool_max: u32,
    ) -> Self {
        DBConnection {
            address,
            port,
            database,
            username,
            password,
            pg_pool_min,
            pg_pool_max,
            pool: None,
        }
    }

    pub fn open_connection(&mut self) {
        let database_url = concat_strs!(
            "postgres://",
            &self.username,
            ':',
            &self.password,
            '@',
            &self.address,
            ':',
            &self.port.to_string(), // Convert a number to string: https://stackoverflow.com/a/24990987/11454077
            '/',
            &self.database
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool_result = Pool::builder()
            .min_idle(Some(self.pg_pool_min))
            .max_size(self.pg_pool_max)
            .build(manager);

        match pool_result {
            Ok(pg_pool) => {
                self.pool = Some(pg_pool);
            }
            Err(error) => {
                log::error!("Error to connect to database: {error}");
            }
        }
    }

    pub fn get_pool(self) -> Option<PgPool> {
        self.pool
    }
}
