use anyhow::{Context, Error};
use std::env;
use std::path::PathBuf;

pub mod configurations;
pub use self::configurations::{ConfigValue, TmcConfig};

pub mod course_config;
pub use self::course_config::*;

// base directory for a given plugin's settings files
pub fn get_tmc_dir(client_name: &str) -> Result<PathBuf, Error> {
    let config_dir = match env::var("TMC_LANGS_CONFIG_DIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => dirs::config_dir().context("Failed to find config directory")?,
    };
    Ok(config_dir.join(format!("tmc-{}", client_name)))
}
