use crate::config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn cnn() -> MysqlConnection {
    dotenv().ok();
    // TODO: Genarics
    let http = match envy::from_env::<config::DB>() {
        Ok(val) => val,
        Err(error) => panic!("{:#?}", error),
    };
    let database_url = &http.database_url;
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
