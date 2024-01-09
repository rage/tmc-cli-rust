//! Wrapper around TmcConfig

use crate::PLUGIN;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    ops::DerefMut,
    path::{Path, PathBuf},
};
use tmc_langs::TmcConfig;
use uuid::Uuid;

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
    pub fn location() -> anyhow::Result<PathBuf> {
        let path = TmcConfig::get_location(PLUGIN)?;
        Ok(path)
    }

    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let config = TmcConfig::load_from(PLUGIN, path)?;
        Ok(Self {
            config,
            mooc_exercises: RefCell::new(None),
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let Self {
            config,
            mooc_exercises: exercises,
        } = self;

        // if exercises have been accessed, they may have been modified
        if let Some(exercises) = exercises.borrow().as_ref() {
            config.insert(
                MOOC_EXERCISES_KEY.to_string(),
                toml::Value::try_from(exercises)?,
            );
        }

        config.save()?;

        Ok(())
    }

    pub fn get_projects_dir(&self) -> &Path {
        self.config.get_projects_dir()
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

    pub fn get_local_mooc_exercises(&self) -> RefMut<'_, Vec<LocalMoocExercise>> {
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

    // ensures no duplicates get added
    pub fn add_mooc_exercise(&mut self, exercise: LocalMoocExercise) {
        let mut exercises = self.get_local_mooc_exercises();
        let existing = exercises.deref_mut().iter_mut().find(|e| e == &&exercise);
        if let Some(existing) = existing {
            *existing = exercise;
        } else {
            exercises.push(exercise);
        }
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
    }
}
