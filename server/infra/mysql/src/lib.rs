use diesel::r2d2::{ConnectionManager, Pool};
use dotenv;
use std::env;

use diesel::mysql::MysqlConnection;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
