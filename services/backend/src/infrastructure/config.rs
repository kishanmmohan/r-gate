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
        let mut postgres_url = String::from("postgres://");
        postgres_url.push_str(&self.postgres_user);
        postgres_url.push_str(":");
        postgres_url.push_str(&self.postgres_password);
        postgres_url.push_str("@");
        postgres_url.push_str(&self.postgres_host);
        postgres_url.push_str(":");
        postgres_url.push_str(&self.postgres_port);
        postgres_url.push_str("/");
        postgres_url.push_str(&self.postgres_db);
        postgres_url
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
    "password".to_string()
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
