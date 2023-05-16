use anyhow::{Context, Result};
use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub port: u16,
    pub api_secret: String,
}

impl Config {
    pub fn get_from_env() -> Result<Config> {
        //load dotenv file if it exists
        match dotenv() {
            Ok(_) => tracing::info!("No .env file found, nothing to load"),
            Err(_) => tracing::info!("Loaded config from .env file"),
        }

        let db_url = dotenvy::var("DATABASE_URL")
            .context("No DB URL provided, cannot connect to database")?;

        //port defaults to 8080 if not provided
        let port = dotenvy::var("port")
            .map_err(Into::<anyhow::Error>::into) //error trait bullshit
            .and_then(|p| p.parse().map_err(Into::into))
            .unwrap_or_else(|_| {
                tracing::warn!("No port provided, defaulting to 8080");
                8080
            });

        //api secret is randomly generated if not provided
        //usually for use in testing
        let api_secret = dotenvy::var("API_SECRET").unwrap_or_else(|_| {
            tracing::warn!("No API secret provided, setting a random one");
            let s = rand_string(32);
            std::env::set_var("API_SECRET", &s);
            tracing::info!("API secret set to {s}");
            s
        });

        let c = Ok(Config {
            db_url,
            port,
            api_secret,
        });

        tracing::info!("Loaded config from environment!");

        c
    }
}

fn rand_string(len: usize) -> String {
    use rand::Rng;

    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
