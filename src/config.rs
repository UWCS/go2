use anyhow::{Context, Result};
use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub port: u16,
    pub api_secret: String,
    pub auth_config: Option<AuthConfig>,
}

#[derive(Debug)]
pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub oidc_url: String,
    pub app_url: String,
}

impl Config {
    pub fn get_from_env() -> Result<Config> {
        //load dotenv file if it exists
        match dotenv() {
            Err(_) => tracing::info!("No .env file found, nothing to load"),
            Ok(_) => tracing::info!("Loaded config from .env file"),
        }

        let db_url = dotenvy::var("DATABASE_URL")
            .context("No DB URL provided, cannot connect to database")?;

        //port defaults to 8080 if not provided
        let port = dotenvy::var("PORT")
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

        let auth_config = match get_auth_config() {
            Ok(conf) => Some(conf),
            Err(e) => {
                tracing::error!("{}, disabling OIDC auth", e);
                None
            }
        };

        let c = Ok(Config {
            db_url,
            port,
            api_secret,
            auth_config,
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

fn get_auth_config() -> anyhow::Result<AuthConfig> {
    let client_id = dotenvy::var("AUTH_CLIENT_ID").context("No client ID provided")?;
    let client_secret = dotenvy::var("AUTH_CLIENT_SECRET").context("No client secret provided")?;
    let oidc_url = dotenvy::var("AUTH_OIDC_URL").context("No OIDC URL provided")?;
    let app_url = dotenvy::var("APP_URL").context("No application URL provided")?;

    Ok(AuthConfig {
        client_id,
        client_secret,
        oidc_url,
        app_url,
    })
}
