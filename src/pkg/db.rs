use crate::pkg::config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn db_connection() -> MysqlConnection {
    let config = config::prepare();
    let database_url = &config.db.url;
    println!("{}", database_url);
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
