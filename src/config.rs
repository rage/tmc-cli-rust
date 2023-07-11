//! Wrapper around TmcConfig

use crate::commands::util;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    ops::DerefMut,
    path::{Path, PathBuf},
};
use tmc_langs::TmcConfig;
use toml::Value;
use uuid::Uuid;

const ORGANIZATION_KEY: &str = "organization";
const TEST_LOGIN_KEY: &str = "test_login";
const TEST_LOGIN_VALUE: &str = "test_logged_in";
const MOOC_EXERCISES_KEY: &str = "mooc_exercises";

pub struct TmcCliConfig {
    config: TmcConfig,
    // lazily deserializes exercises when needed
    // the refcell usage should be such that it can't panic...
    mooc_exercises: RefCell<Option<Vec<LocalMoocExercise>>>,
}

impl TmcCliConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = TmcConfig::load(util::PLUGIN)?;
        Ok(Self {
            config,
            mooc_exercises: RefCell::new(None),
        })
    }

    pub fn save(self) -> anyhow::Result<()> {
        let Self {
            mut config,
            mooc_exercises: exercises,
        } = self;

        // if exercises have been accessed, they may have been modified
        if let Some(exercises) = exercises.borrow().as_ref() {
            config.insert(MOOC_EXERCISES_KEY.to_string(), Value::try_from(exercises)?);
        }

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

    pub fn get_mooc_exercises(&self) -> RefMut<'_, Vec<LocalMoocExercise>> {
        self.init_mooc_exercises()
    }

    // ensures no duplicates get added
    pub fn add_mooc_exercise(&mut self, exercise: LocalMoocExercise) {
        let mut exercises = self.init_mooc_exercises();
        let existing = exercises.deref_mut().iter_mut().find(|e| e == &&exercise);
        if let Some(existing) = existing {
            *existing = exercise;
        } else {
            exercises.push(exercise);
        }
    }

    fn init_mooc_exercises(&self) -> RefMut<'_, Vec<LocalMoocExercise>> {
        RefMut::map(self.mooc_exercises.borrow_mut(), |ex| match ex {
            Some(exercises) => exercises,
            None => {
                let exercises = self
                    .config
                    .get(MOOC_EXERCISES_KEY)
                    .and_then(|v| v.clone().try_into().ok())
                    .unwrap_or_default();
                ex.insert(exercises)
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalMoocExercise {
    pub course_instance_id: Uuid,

    pub exercise_name: String,
    pub exercise_id: Uuid,
    pub slide_id: Uuid,
    pub task_id: Uuid,

    pub location: PathBuf,
    pub download_url: String,
    pub checksum: String,
}

/// Two exercises are considered equal if they have the same task ids
// (we could check exercise/slide ids here but we can rely on UUIDs being unique)
impl PartialEq for LocalMoocExercise {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
    }
}
