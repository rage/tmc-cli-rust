use anyhow::Context;
use file_util::create_file_lock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use tmc_langs_framework::file_util;

/// Save configurations, like organization, in JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: PathBuf,
    settings: Value,
}

impl Config {
    fn get_config_path(client_name: &str) -> anyhow::Result<PathBuf> {
        super::get_tmc_dir(client_name).map(|dir| dir.join("config.json"))
    }

    /// ### Returns
    /// - Ok(Some) if a config file exists and can be deserialized,
    /// - Ok(None) if no config file exists, and
    /// - Err if a config file exists but cannot be deserialized.
    ///
    /// On Err, the file is deleted.
    ///
    pub fn load(client_name: &str) -> anyhow::Result<Self> {
        let config_path = Self::get_config_path(client_name).unwrap();
        if !config_path.exists() {
            anyhow::bail!("Config file path does not exist!");
        }

        if let Ok(config_str) = file_util::read_file_to_string(&config_path) {
            let result = match serde_json::from_str(&config_str) {
                Ok(json) => Ok(Config {
                    path: config_path,
                    settings: json,
                }),
                Err(e) => {
                    log::error!(
                        "Failed to deserialize credentials.json due to \"{}\", deleting",
                        e
                    );
                    fs::remove_file(&config_path).with_context(|| {
                        format!(
                            "Failed to remove malformed credentials.json file {}",
                            config_path.display()
                        )
                    })?;
                    anyhow::bail!(
                        "Failed to deserialize credentials file at {}; removed the file, please try again.",
                        config_path.display()
                        )
                }
            };

            return result;
        }

        Ok(Config {
            path: config_path,
            settings: serde_json::json!({}),
        })
    }

    pub fn get_value(&self, key: &str) -> anyhow::Result<String> {
        let value: String = serde_json::from_value(self.settings[key].clone())?;
        return Ok(value);
    }

    pub fn change_value(&mut self, key: &str, new_val: &str) -> anyhow::Result<()> {
        let mut config: HashMap<String, Value> = serde_json::from_value(self.settings.clone())?;

        config.insert(key.to_string(), Value::from(new_val));

        self.settings = serde_json::to_value(config)?;
        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = &self.path;

        println!("{:?}", config_path);

        if let Some(p) = config_path.parent() {
            fs::create_dir_all(p)
                .with_context(|| format!("Failed to create directory {}", p.display()))?;
        }

        let mut config_file = create_file_lock(&config_path)
            .with_context(|| format!("Failed to create file at {}", config_path.display()))?;
        let guard = config_file.lock()?;

        // write token
        if let Err(e) = serde_json::to_writer(guard.deref(), &self.settings) {
            // failed to write token, removing config file
            fs::remove_file(&config_path).with_context(|| {
                format!(
                    "Failed to remove empty config file after failing to write {}",
                    config_path.display()
                )
            })?;
            Err(e)
                .with_context(|| format!("Failed to write config to {}", config_path.display()))?;
        }
        Ok(())
    }

    //pub fn remove(self) -> Result<()> {
    //file_util::lock!(&self.path);

    //fs::remove_file(&self.path)
    //.with_context(|| format!("Failed to remove config at {}", self.path.display()))?;
    //Ok(())
    //}
    //Ok(())
    //}
}
