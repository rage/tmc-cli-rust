//! Wrapper around TmcConfig

use crate::PLUGIN;
use std::path::{Path, PathBuf};
use tmc_langs::TmcConfig;

const ORGANIZATION_KEY: &str = "organization";
const TEST_LOGIN_KEY: &str = "test_login";
const TEST_LOGIN_VALUE: &str = "test_logged_in";

pub struct TmcCliConfig {
    config: TmcConfig,
}

impl TmcCliConfig {
    pub fn location() -> anyhow::Result<PathBuf> {
        let path = TmcConfig::get_location(PLUGIN)?;
        Ok(path)
    }

    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let config = TmcConfig::load_from(PLUGIN, path)?;
        Ok(Self { config })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let Self { config } = self;

        config.save()?;

        Ok(())
    }

    pub fn get_projects_dir(&self) -> &Path {
        self.config.get_projects_dir()
    }

    pub fn get_organization(&self) -> Option<&str> {
        self.config.get(ORGANIZATION_KEY).and_then(|v| v.as_str())
    }

    pub fn set_organization(&mut self, org: String) {
        self.config
            .insert(ORGANIZATION_KEY.to_string(), toml::Value::String(org));
    }

    pub fn get_test_login(&self) -> Option<&str> {
        self.config.get(TEST_LOGIN_KEY).and_then(|v| v.as_str())
    }

    pub fn set_test_login(&mut self) {
        let key = TEST_LOGIN_KEY.to_string();
        let value = toml::Value::String(TEST_LOGIN_VALUE.to_string());
        self.config.insert(key, value);
    }

    pub fn remove_test_login(&mut self) {
        self.config.remove(TEST_LOGIN_KEY);
    }
}

#[cfg(target_os = "windows")]
impl TmcCliConfig {
    const UPDATE_LAST_CHECKED_KEY: &str = "update-last-checked";

    pub fn get_update_last_checked(&self) -> Option<u128> {
        self.config
            .get(Self::UPDATE_LAST_CHECKED_KEY)
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u128>().ok())
    }

    pub fn update_last_checked(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let key = Self::UPDATE_LAST_CHECKED_KEY.to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Invalid system time")
            .as_millis();
        self.config
            .insert(key, toml::Value::String(timestamp.to_string()));
    }
}
