use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BaseSettings {
    #[serde(default = "default_postgres_user")]
    pub postgres_user: String,

    #[serde(default = "default_postgres_password")]
    pub postgres_password: String,

    #[serde(default = "default_postgres_host")]
    pub postgres_host: String,

    #[serde(default = "default_postgres_port")]
    pub postgres_port: String,

    #[serde(default = "default_postgres_db")]
    pub postgres_db: String,

}

impl BaseSettings{

    pub fn construct_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }
}


pub fn base_settings() -> BaseSettings {
    match envy::from_env::<BaseSettings>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}

fn default_postgres_user() -> String {
    "postgres".to_string()
}

fn default_postgres_password() -> String {
    "rgate".to_string()
}


fn default_postgres_host() -> String {
    "localhost".to_string()
}

fn default_postgres_port() -> String {
    "5432".to_string()
}

fn default_postgres_db() -> String {
    "IAM".to_string()
}
