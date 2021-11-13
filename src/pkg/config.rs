use dotenv::dotenv;
use serde::Deserialize;

serde_with::with_prefix!(phttp "http_");
serde_with::with_prefix!(pdb "db_");
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(flatten, with = "phttp")]
    pub http: HTTP,
    #[serde(flatten, with = "pdb")]
    pub db: DB,
}

#[derive(Deserialize, Debug)]
pub struct HTTP {
    pub host: String,
    pub port: String,
}

#[derive(Deserialize, Debug)]
pub struct DB {
    pub url: String,
}

pub fn prepare() -> Config {
    dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(error) => panic!("{:#?}", error),
    };
    config
}
