use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{borrow::Cow, fs};
use tmc_langs::file_util;
use toml::{value::Table, Value};

/// Save configurations, like organization, in JSON
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TmcConfig {
    pub projects_dir: PathBuf,
    #[serde(flatten)]
    pub table: Table,
    client_name: String,
}

impl TmcConfig {
    fn get_config_path(client_name: &str) -> Result<PathBuf> {
        super::get_tmc_dir(client_name).map(|dir| dir.join("config.toml"))
    }

    pub fn get(&self, key: &str) -> ConfigValue {
        match key {
            "projects-dir" => ConfigValue::Path(Cow::Borrowed(&self.projects_dir)),
            _ => ConfigValue::Value(self.table.get(key).map(Cow::Borrowed)),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) -> Result<()> {
        match key.as_str() {
            "projects-dir" => {
                if let Value::String(value) = value {
                    let path = PathBuf::from(value);
                    self.set_projects_dir(path)?;
                } else {
                    anyhow::bail!("The value for projects-dir must be a string.")
                }
            }
            _ => {
                self.table.insert(key, value);
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<Value>> {
        match key {
            "projects-dir" => anyhow::bail!("projects-dir must always be defined"),
            _ => Ok(self.table.remove(key)),
        }
    }

    pub fn set_projects_dir(&mut self, mut target: PathBuf) -> Result<PathBuf> {
        // check if the directory is empty or not
        if fs::read_dir(&target)
            .with_context(|| format!("Failed to read directory at {}", target.display()))?
            .next()
            .is_some()
        {
            anyhow::bail!("Cannot set projects-dir to a non-empty directory.");
        }
        std::mem::swap(&mut self.projects_dir, &mut target);
        Ok(target)
    }

    pub fn save(self) -> Result<()> {
        let path = TmcConfig::get_config_path(&self.client_name).unwrap();
        if let Some(parent) = path.parent() {
            file_util::create_dir_all(parent)?;
        }
        let mut lock = file_util::create_file_lock(&path)?;
        let mut guard = lock.lock()?;

        let toml = toml::to_string_pretty(&self).context("Failed to serialize HashMap")?;
        guard
            .write_all(toml.as_bytes())
            .with_context(|| format!("Failed to write TOML to {}", path.display()))?;
        Ok(())
    }

    pub fn reset(client_name: &'static str) -> Result<()> {
        let path = Self::get_location(client_name)?;
        Self::init_at(client_name, &path)?; // init locks the file
        Ok(())
    }

    pub fn load(client_name: &'static str) -> Result<TmcConfig> {
        // try to open config file

        let path = TmcConfig::get_config_path(client_name).unwrap();
        let config = match file_util::open_file_lock(&path) {
            Ok(mut lock) => {
                // found config file, lock and read
                let mut guard = lock.lock()?;
                let mut buf = vec![];
                let _bytes = guard.read_to_end(&mut buf)?;
                match toml::from_slice(&buf) {
                    // successfully read file, try to deserialize
                    Ok(config) => config, // successfully read and deserialized the config
                    Err(_) => {
                        log::error!(
                            "Failed to deserialize config at {}, resetting",
                            path.display()
                        );
                        Self::init_at(client_name, &path)?
                    }
                }
            }
            Err(e) => {
                // failed to open config file, create new one
                log::info!(
                    "could not open config file at {} due to {}, initializing a new config file",
                    path.display(),
                    e
                );
                // todo: check the cause to make sure this makes sense, might be necessary to propagate some error kinds
                Self::init_at(client_name, &path)?
            }
        };

        if !config.projects_dir.exists() {
            fs::create_dir_all(&config.projects_dir).with_context(|| {
                format!(
                    "Failed to create projects-dir at {}",
                    config.projects_dir.display()
                )
            })?;
        }
        Ok(config)
    }

    // initializes the default configuration file at the given path
    fn init_at(client_name: &'static str, path: &Path) -> Result<TmcConfig> {
        if let Some(parent) = path.parent() {
            file_util::create_dir_all(parent)?;
        }

        let mut lock = file_util::create_file_lock(path)
            .with_context(|| format!("Failed to create new config file at {}", path.display()))?;
        let mut guard = lock.lock()?;

        let default_project_dir = dirs::data_local_dir()
            .context("Failed to find local data directory")?
            .join("tmc")
            .join(Self::get_client_stub(client_name));
        fs::create_dir_all(&default_project_dir).with_context(|| {
            format!(
                "Failed to create the TMC default project directory in {}",
                default_project_dir.display()
            )
        })?;

        let config = TmcConfig {
            projects_dir: default_project_dir,
            table: Table::new(),
            client_name: client_name.to_string(),
        };

        let toml = toml::to_string_pretty(&config).context("Failed to serialize config")?;
        guard
            .write_all(toml.as_bytes())
            .with_context(|| format!("Failed to write default config to {}", path.display()))?;
        Ok(config)
    }

    // path to the configuration file
    pub fn get_location(client_name: &str) -> Result<PathBuf> {
        super::get_tmc_dir(client_name).map(|dir| dir.join("config.toml"))
    }

    // some clients use a different name for the directory
    fn get_client_stub(client: &str) -> &str {
        match client {
            "vscode_plugin" => "vscode",
            s => s,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ConfigValue<'a> {
    Value(Option<Cow<'a, Value>>),
    Path(Cow<'a, Path>),
}

impl ConfigValue<'_> {
    pub fn into_owned(self) -> ConfigValue<'static> {
        match self {
            Self::Value(Some(v)) => ConfigValue::Value(Some(Cow::Owned(v.into_owned()))),
            Self::Value(None) => ConfigValue::Value(None),
            Self::Path(p) => ConfigValue::Path(Cow::Owned(p.into_owned())),
        }
    }
}
