//! Wrapper around TmcConfig

use crate::commands::util;
use std::path::Path;
use tmc_langs::TmcConfig;

const ORGANIZATION_KEY: &str = "organization";
const TEST_LOGIN_KEY: &str = "test_login";
const TEST_LOGIN_VALUE: &str = "test_logged_in";

pub struct TmcCliConfig {
    config: TmcConfig,
}

impl TmcCliConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = TmcConfig::load(util::PLUGIN)?;
        Ok(Self { config })
    }

    pub fn save(self) -> anyhow::Result<()> {
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

    pub fn insert_organization(&mut self, org: String) {
        self.config
            .insert(ORGANIZATION_KEY.to_string(), toml::Value::String(org));
    }

    pub fn get_test_login(&self) -> Option<&str> {
        self.config.get(TEST_LOGIN_KEY).and_then(|v| v.as_str())
    }

    pub fn insert_test_login(&mut self) {
        let key = TEST_LOGIN_KEY.to_string();
        let value = toml::Value::String(TEST_LOGIN_VALUE.to_string());
        self.config.insert(key, value);
    }

    pub fn remove_test_login(&mut self) {
        let key = TEST_LOGIN_KEY;
        self.config.remove(key);
    }
}
