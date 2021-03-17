use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use tmc_client::CourseDetails;
use tmc_client::Exercise;
use tmc_client::Organization;

pub const COURSE_CONFIG_FILE_NAME: &str = ".tmc.json";

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CourseDetailsWrapper {
    pub id: usize,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub exercises: Vec<Exercise>,
    pub details_url: String,
    pub unlock_url: String,
    pub comet_url: String,
    pub spyware_urls: Vec<String>,
    pub reviews_url: String,
    pub unlockables: Vec<String>,
    pub exercisesLoaded: bool, // Not used. Exists to be compatible with java tmc.
}

impl CourseDetailsWrapper {
    pub fn new(cd: CourseDetails) -> CourseDetailsWrapper {
        CourseDetailsWrapper {
            id: cd.course.id,
            name: cd.course.name,
            title: cd.course.title,
            description: cd.course.description,
            exercises: cd.exercises,
            details_url: cd.course.details_url,
            unlock_url: cd.course.unlock_url,
            comet_url: cd.course.comet_url,
            spyware_urls: cd.course.spyware_urls,
            reviews_url: cd.course.reviews_url,
            unlockables: cd.unlockables,
            exercisesLoaded: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseConfig {
    pub username: String,
    pub server_address: String,
    pub course: CourseDetailsWrapper,
    pub organization: Organization,
    pub local_completed_exercises: Vec<String>, // NotImplemented
    pub properties: HashMap<String, String>,    // NotImplemented
}

/// Loads course information from file
pub fn load_course_config(path: &Path) -> Result<CourseConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

/// Saves course information to file
pub fn save_course_information(course_config: CourseConfig, pathbuf: PathBuf) {
    let f = File::create(pathbuf).expect("Unable to create file");
    let bw = BufWriter::new(f);
    serde_json::to_writer(bw, &course_config).expect("Failed writing course information");
}

pub fn get_exercise_by_id(course_config: &CourseConfig, id: usize) -> Option<&Exercise> {
    for exercise in &course_config.course.exercises {
        if exercise.id == id {
            return Some(exercise);
        }
    }
    None
}

pub fn get_exercise_by_name<'a>(
    course_config: &'a CourseConfig,
    name: &str,
) -> Option<&'a Exercise> {
    for exercise in &course_config.course.exercises {
        if exercise.name == name {
            return Some(exercise);
        }
    }
    None
}
