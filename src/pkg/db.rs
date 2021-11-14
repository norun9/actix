use crate::pkg::config::Config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn db_connection() -> MysqlConnection {
    let config = Config::prepare();
    let database_url = &config.db.url;
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
