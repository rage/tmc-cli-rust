//! Wrapper around TmcConfig

use crate::commands::util;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tmc_langs::TmcConfig;
use toml::Value;
use uuid::Uuid;

pub struct TmcCliConfig {
    config: TmcConfig,
    added_exercises: Vec<LocalExercise>,
}

impl TmcCliConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = TmcConfig::load(util::PLUGIN)?;
        Ok(Self {
            config,
            added_exercises: Vec::new(),
        })
    }

    pub fn save(self) -> anyhow::Result<()> {
        let Self {
            mut config,
            added_exercises,
        } = self;
        config.insert("exercises".to_string(), Value::try_from(added_exercises)?);
        config.save()?;
        Ok(())
    }

    pub fn get_organization(&self) -> Option<&str> {
        self.config.get("organization").and_then(|v| v.as_str())
    }

    pub fn insert_organization(&mut self, org: String) {
        self.config
            .insert("organization".to_string(), toml::Value::String(org));
    }

    pub fn get_test_login(&self) -> Option<&str> {
        self.config.get("test_login").and_then(|v| v.as_str())
    }

    pub fn insert_test_login(&mut self) {
        let key = "test_login".to_string();
        let value = toml::Value::String("test_logged_in".to_string());
        self.config.insert(key, value);
    }

    pub fn remove_test_login(&mut self) {
        let key = "test_login";
        self.config.remove(key);
    }

    pub fn add_exercise(&mut self, exercise: LocalExercise) {
        self.added_exercises.push(exercise);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalExercise {
    pub exercise_id: Uuid,
    pub slide_id: Uuid,
    pub task_id: Uuid,
    pub location: PathBuf,
}
