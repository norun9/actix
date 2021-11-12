use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HTTP {
    pub host: String,
    pub port: String,
}

#[derive(Deserialize, Debug)]
pub struct DB {
    pub database_url: String,
}
