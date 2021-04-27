use isolang::Language;
use reqwest::Url;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use tmc_client::{
    ClientError, Course, CourseDetails, CourseExercise, ExercisesDetails, NewSubmission,
    Organization, SubmissionFinished, TmcClient, Token,
};
use tmc_langs::file_util;
use tmc_langs::Credentials;
use tmc_langs::DownloadResult;
use tmc_langs::LangsError;
use tmc_langs::{ConfigValue, CourseConfig, ProjectsConfig, TmcConfig};
use toml::de::Error;

pub const PLUGIN: &str = "tmc_cli_rust";
pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct ClientProduction {
    pub tmc_client: TmcClient,
    pub test_mode: bool,
}

use mockall::predicate::*;
use mockall::*;

use crate::interactive::interactive_list;
#[automock]
pub trait Client {
    fn load_login(&mut self) -> Result<(), String>;
    fn try_login(&mut self, username: String, password: String) -> Result<String, String>;
    fn list_courses(&mut self) -> Result<Vec<Course>, String>;
    fn get_organizations(&mut self) -> Result<Vec<Organization>, String>;
    fn logout(&mut self);
    fn wait_for_submission(&self, submission_url: &str) -> Result<SubmissionFinished, ClientError>;
    fn submit(
        &self,
        submission_url: Url,
        submission_path: &Path,
        locale: Option<Language>,
    ) -> Result<NewSubmission, ClientError>;
    fn get_course_exercises(&mut self, course_id: usize) -> Result<Vec<CourseExercise>, String>;
    fn get_exercise_details(
        &mut self,
        exercise_ids: Vec<usize>,
    ) -> Result<Vec<ExercisesDetails>, String>;
    fn download_or_update_exercises(
        &mut self,
        download_params: &[usize],
        path: &Path,
    ) -> Result<DownloadResult, LangsError>;
    fn is_test_mode(&mut self) -> bool;
    fn get_course_details(&self, course_id: usize) -> Result<CourseDetails, ClientError>;
    fn get_organization(&self, organization_slug: &str) -> Result<Organization, ClientError>;
    fn paste(
        &self,
        submission_url: Url,
        submission_path: &Path,
        paste_message: Option<String>,
        locale: Option<Language>,
    ) -> Result<NewSubmission, String>;
}

static SERVER_ADDRESS: &str = "https://tmc.mooc.fi";
impl ClientProduction {
    pub fn new(test_mode: bool) -> Self {
        let (tmc_client, _credentials) = tmc_langs::init_tmc_client_with_credentials(
            SERVER_ADDRESS.to_string(),
            PLUGIN,
            "1.0.0",
        )
        .unwrap();

        ClientProduction {
            tmc_client,
            test_mode,
        }
    }
    #[allow(dead_code)]
    pub fn is_test_mode(&mut self) -> bool {
        self.test_mode
    }

    fn authenticate(&mut self, username: String, password: String) -> Result<Token, String> {
        match self.tmc_client.authenticate(PLUGIN, username, password) {
            Ok(x) => Ok(x),
            Err(x) => Err(ClientProduction::explain_login_fail(x)),
        }
    }

    fn explain_login_fail(error: ClientError) -> String {
        let res = format!("{:?}", error);

        if res.contains("The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client.") {
            return WRONG_LOGIN.to_string();
        }

        "Login failed with an unknown error message".to_string()
    }
}

impl Client for ClientProduction {
    fn paste(
        &self,
        submission_url: Url,
        submission_path: &Path,
        paste_message: Option<String>,
        locale: Option<Language>,
    ) -> Result<NewSubmission, String> {
        if self.test_mode {
            return Err("Integration test input not yet implemented for paste command".to_string());
        }
        match self
            .tmc_client
            .paste(submission_url, submission_path, paste_message, locale)
        {
            Err(client_error) => match client_error {
                ClientError::HttpError {
                    url: _,
                    status,
                    error,
                    obsolete_client: _,
                } => Err(format!("Status {}, message: {}", status, error)),
                _ => Err(
                    "Received unhandled ClientError when calling paste command from tmc_client"
                        .to_string(),
                ),
            },
            Ok(submission) => Ok(submission),
        }
    }

    fn is_test_mode(&mut self) -> bool {
        self.test_mode
    }

    fn load_login(&mut self) -> Result<(), String> {
        if self.test_mode {
            // Test login exists if config-file has key-value pair test_login = "test_logged_in"
            let config = TmcConfig::load(PLUGIN, &get_path()).unwrap();
            let test_login_exists = match config.get("test_login") {
                ConfigValue::Value(Some(value)) => {
                    toml::Value::as_str(&value).unwrap() == "test_logged_in"
                }
                _ => false,
            };
            if test_login_exists {
                return Ok(());
            } else {
                return Err(
                    "No login found. You need to be logged in to use this command".to_string(),
                );
            }
        }

        if let Some(credentials) = get_credentials() {
            match self.tmc_client.set_token(credentials.token()) {
                Ok(()) => return Ok(()),
                _ => return Err("Setting login token failed".to_string()),
            }
        }
        Err("No login found. You need to be logged in to use this command".to_string())
    }

    fn try_login(&mut self, username: String, password: String) -> Result<String, String> {
        let token;

        if self.test_mode {
            if username == "testusername" && password == "testpassword" {
                let mut config = TmcConfig::load(PLUGIN, &get_path()).unwrap();

                if let Err(_err) = config.insert(
                    "test_login".to_string(),
                    toml::Value::String("test_logged_in".to_string()),
                ) {
                    return Err("Test login value could not be changed in config file".to_string());
                }

                if let Err(_err) = config.save(&get_path()) {
                    return Err("Problem saving login".to_string());
                }

                return Ok(SUCCESSFUL_LOGIN.to_string());
            }
            return Err(WRONG_LOGIN.to_string());
        }
        match self.authenticate(username, password) {
            Ok(x) => token = x,
            Err(x) => return Err(x),
        }

        if Credentials::save(PLUGIN, token).is_ok() {
            return Ok(SUCCESSFUL_LOGIN.to_string());
        };

        Err("Error! Saving credentials failed".to_string())
    }

    fn list_courses(&mut self) -> Result<Vec<Course>, String> {
        if self.test_mode {
            return Ok(vec![
                Course {
                    name: "test-tmc-test-course".to_string(),
                    id: 0,

                    title: "".to_string(),
                    description: None,
                    details_url: "".to_string(),
                    unlock_url: "".to_string(),
                    reviews_url: "".to_string(),
                    comet_url: "".to_string(),
                    spyware_urls: vec![],
                },
                Course {
                    name: "imaginary-test-course".to_string(),
                    id: 1,

                    title: "".to_string(),
                    description: None,
                    details_url: "".to_string(),
                    unlock_url: "".to_string(),
                    reviews_url: "".to_string(),
                    comet_url: "".to_string(),
                    spyware_urls: vec![],
                },
            ]);
        }

        match self.tmc_client.list_courses(&get_organization().unwrap()) {
            Ok(courses) => {
                let mut course_list: Vec<Course> = Vec::new();
                for course in courses {
                    course_list.push(Course {
                        name: course.name,
                        id: course.id,

                        title: "".to_string(),
                        description: None,
                        details_url: "".to_string(),
                        unlock_url: "".to_string(),
                        reviews_url: "".to_string(),
                        comet_url: "".to_string(),
                        spyware_urls: vec![],
                    });
                }
                Ok(course_list)
            }
            Err(ClientError::NotLoggedIn) => {
                Err("Login token is invalid. Please try logging in again.".to_string())
            }
            _ => Err("Unknown error. Please try again.".to_string()),
        }
    }

    fn get_organizations(&mut self) -> Result<Vec<Organization>, String> {
        if self.test_mode {
            return Ok(vec![
                Organization {
                    name: "test organization".to_string(),
                    slug: "test".to_string(),

                    information: "".to_string(),
                    logo_path: "".to_string(),
                    pinned: false,
                },
                Organization {
                    name: "imaginary test organization".to_string(),
                    slug: "imag".to_string(),

                    information: "".to_string(),
                    logo_path: "".to_string(),
                    pinned: false,
                },
            ]);
        }
        let result = self.tmc_client.get_organizations();
        match result {
            Ok(organizations) => {
                let mut org_list: Vec<Organization> = Vec::new();
                for org in organizations {
                    org_list.push(org);
                }
                Ok(org_list)
            }
            _ => Err("Could not get organizations from server".to_string()),
        }
    }

    fn logout(&mut self) {
        if self.test_mode {
            // Remove test login from config file
            let mut config = match TmcConfig::load(PLUGIN, &get_path()) {
                Ok(config) => config,
                _ => panic!("Could not load the config"),
            };
            if let Err(_err) = config.remove("test_login") {
                panic!("Could not remove test login from config in test mode");
            }
            if let Err(_err) = config.save(&get_path()) {
                panic!("Could not save config after removing test login in test mode");
            }
            return;
        }

        let credentials = get_credentials().unwrap();

        credentials.remove().unwrap();
    }

    fn wait_for_submission(&self, submission_url: &str) -> Result<SubmissionFinished, ClientError> {
        self.tmc_client.wait_for_submission(submission_url)
    }
    fn submit(
        &self,
        submission_url: Url,
        submission_path: &Path,
        locale: Option<Language>,
    ) -> Result<NewSubmission, ClientError> {
        if self.test_mode {
            return Ok(NewSubmission {
                show_submission_url: "https://tmc.mooc.fi/submissions/7400888".to_string(),
                paste_url: "url".to_string(),
                submission_url: "https://tmc.mooc.fi/api/v8/core/submissions/7400888".to_string(),
            });
        }
        self.tmc_client
            .submit(submission_url, submission_path, locale)
    }

    fn get_course_exercises(&mut self, course_id: usize) -> Result<Vec<CourseExercise>, String> {
        if self.test_mode {
            return Ok(vec![CourseExercise {
                id: 0,
                available_points: vec![],
                awarded_points: vec![],
                name: "Imaginary test exercise".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: false,
                unlocked: true,
            }]);
        }
        match self.tmc_client.get_course_exercises(course_id) {
            Ok(exercises) => Ok(exercises),
            Err(ClientError::NotLoggedIn) => {
                Err("Login token is invalid. Please try logging in again.".to_string())
            }
            _ => Err("Unknown error. Please try again.".to_string()),
        }
    }

    fn get_exercise_details(
        &mut self,
        exercise_ids: Vec<usize>,
    ) -> Result<Vec<ExercisesDetails>, String> {
        if self.test_mode {
            return Ok(vec![ExercisesDetails {
                id: 0,
                course_name: "test_course".to_string(),
                exercise_name: "test_exercise".to_string(),
                checksum: "test_checksum".to_string(),
            }]);
        }
        match self.tmc_client.get_exercises_details(&exercise_ids) {
            Ok(exercise_details) => Ok(exercise_details),
            Err(_) => Err("Unknown error. Please try again.".to_string()),
        }
    }

    fn download_or_update_exercises(
        &mut self,
        exercise_ids: &[usize],
        path: &Path,
    ) -> Result<DownloadResult, LangsError> {
        if self.test_mode {
            return Ok(DownloadResult::Success {
                downloaded: vec![],
                skipped: vec![],
            });
        }

        tmc_langs::download_or_update_course_exercises(&self.tmc_client, path, &exercise_ids, true)
    }

    fn get_course_details(&self, course_id: usize) -> Result<CourseDetails, ClientError> {
        if self.test_mode {
            let course = Course {
                id: 0,
                name: "".to_string(),
                title: "".to_string(),
                description: None,
                details_url: "".to_string(),
                unlock_url: "".to_string(),
                reviews_url: "".to_string(),
                comet_url: "".to_string(),
                spyware_urls: vec![],
            };
            Ok(CourseDetails {
                course,
                unlockables: vec![],
                exercises: vec![],
            })
        } else {
            self.tmc_client.get_course_details(course_id)
        }
    }
    fn get_organization(
        &self,
        organization_slug: &str,
    ) -> std::result::Result<Organization, tmc_client::ClientError> {
        if self.test_mode {
            return Ok(Organization {
                name: "String".to_string(),
                information: "String".to_string(),
                slug: "String".to_string(),
                logo_path: "String".to_string(),
                pinned: false,
            });
        }
        self.tmc_client.get_organization(organization_slug)
    }
}

pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap_or(None)
}

// Returns slug of organization as String (if successful)
#[allow(dead_code)]
pub fn get_organization() -> Option<String> {
    match TmcConfig::load(PLUGIN, &get_path()) {
        Ok(config) => {
            // convert the toml::Value to String (if possible)
            match config.get("organization") {
                ConfigValue::Value(Some(value)) => {
                    Some(toml::Value::as_str(&value).unwrap().to_string())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn set_organization(org: &str) -> Result<(), String> {
    let mut config = match TmcConfig::load(PLUGIN, &get_path()) {
        Ok(config) => config,
        _ => return Err("Config could not be loaded".to_string()),
    };

    if let Err(_err) = config.insert(
        "organization".to_string(),
        toml::Value::String(org.to_string()),
    ) {
        return Err("Organization could not be changed".to_string());
    }

    if let Err(_err) = config.save(&get_path()) {
        return Err("Problem saving configurations".to_string());
    }
    Ok(())
}

/// Returns course id as: Ok(Some(usize)) or Ok(None) if not found, Err(msg) if could not get id list
pub fn get_course_id_by_name(
    client: &mut dyn Client,
    course_name: String,
) -> Result<Option<usize>, String> {
    match client.list_courses() {
        Ok(courses) => {
            for course in courses {
                if course.name == course_name {
                    return Ok(Some(course.id));
                }
            }
            Ok(None)
        }
        Err(msg) => Err(msg),
    }
}

/// Returns course as: Ok(Some(Course)) or Ok(None) if not found, Err(msg) if could not get courses list
pub fn get_course_by_name(
    client: &mut dyn Client,
    course_name: String,
) -> Result<Option<Course>, String> {
    match client.list_courses() {
        Ok(courses) => {
            for course in courses {
                if course.name == course_name {
                    return Ok(Some(course));
                }
            }
            Ok(None)
        }
        Err(msg) => Err(msg),
    }
}

static CONFIG_FILE_NAME: &str = "course_config.toml";

/// Checks if current directory or given path
/// contains valid exercise (i.e config file)
/// Returns Err(msg) if given invalid path (including root)
/// Returns Ok(()) if no path given, but if current dir is not
/// an exercise, leaves course_config as None
pub fn find_course_config_for_exercise(
    exercise_slug: &mut String,
    course_config: &mut Option<CourseConfig>,
    exercise_dir: &mut PathBuf,
    path: &str,
) -> Result<(), String> {
    if path.is_empty() {
        // No exercise path given, so assuming we are in exercise directory.
        *exercise_slug = match env::current_dir().unwrap().file_name() {
            Some(file_name) => file_name.to_str().unwrap().to_string(),
            None => return Ok(()),
        };
        let mut pathbuf = env::current_dir().unwrap();
        pathbuf.pop(); // we go to the course directory
        pathbuf.push(CONFIG_FILE_NAME);
        *course_config = match read_new_course_config(pathbuf.as_path()) {
            Ok(conf) => match conf {
                Some(config) => Some(config),
                None => return Ok(()),
            },
            Err(_) => {
                *course_config = None;
                return Ok(());
            }
        };
        *exercise_dir = env::current_dir().unwrap();
    } else {
        // Path given, find out course part, exercise name and full path
        *exercise_slug = match Path::new(path).to_path_buf().file_name() {
            Some(file_name) => file_name.to_str().unwrap().to_string(),
            None => return Err("Invalid exercise path given".to_string()),
        };
        let mut part_path = Path::new(path).to_path_buf();
        part_path.pop();
        let mut course_config_path = env::current_dir().unwrap();
        course_config_path.push(part_path);
        course_config_path.push(CONFIG_FILE_NAME);
        *course_config = match read_new_course_config(course_config_path.as_path()) {
            Ok(conf) => match conf {
                Some(config) => Some(config),
                None => return Err("Invalid exercise path given".to_string()),
            },
            Err(msg) => {
                return Err(msg);
            }
        };
        *exercise_dir = env::current_dir().unwrap();
        exercise_dir.push(Path::new(path).to_path_buf());
    }
    Ok(())
}

/// Reads config file from path. Returns Ok(CourseConfig) if successful,
/// Ok(None) if file not found, Err(msg) if error in parsing/reading file
pub fn read_new_course_config(course_config_path: &Path) -> Result<Option<CourseConfig>, String> {
    if course_config_path.exists() {
        let bytes_result = file_util::read_file(course_config_path);

        if let Ok(bytes) = bytes_result {
            let course_config_result: Result<CourseConfig, Error> = toml::from_slice(&bytes);
            if let Ok(course_config) = course_config_result {
                Ok(Some(course_config))
            } else {
                Err("error parsing course config file".to_string())
            }
        } else {
            Err("error reading course config file".to_string())
        }
    } else {
        Ok(None)
    }
}

/// Retrieves exercise id for exercise from CourseConfig
pub fn get_exercise_id_from_config(
    course_config: &CourseConfig,
    exercise_slug: &str,
) -> Result<usize, String> {
    if course_config.exercises.contains_key(exercise_slug) {
        Ok(course_config.exercises[exercise_slug].id)
    } else {
        Err("could not find exercise in course config".to_string())
    }
}

/// Generates return_url for submissions and pastes
pub fn generate_return_url(exercise_id: usize) -> String {
    format!(
        "{}/api/v8/core/exercises/{}/submissions",
        SERVER_ADDRESS, exercise_id
    )
}

pub fn get_path() -> PathBuf {
    TmcConfig::get_location(PLUGIN).unwrap()
}

pub fn get_projects_dir() -> PathBuf {
    tmc_langs::get_projects_dir(PLUGIN).unwrap()
}

/// Choose course and then exercise interactively, return exercise path
/// or Err(String) if either menu is interrupted or no items found
pub fn choose_exercise() -> Result<PathBuf, String> {
    let mut courses: Vec<String> = Vec::new();

    let projects_config = match ProjectsConfig::load(&get_projects_dir()) {
        Ok(projects_config) => projects_config,
        Err(_err) => return Err(String::from("Could not load info about projects")),
    };

    for course in &projects_config.courses {
        courses.push(course.0.clone());
    }

    if courses.is_empty() {
        return Err(format!(
            "No courses found from current or project directory. Project directory set to {}",
            get_projects_dir().to_str().unwrap().to_string()
        ));
    }

    let chosen_course = match interactive_list("First select course: ", courses) {
        Some(selection) => selection,
        None => return Err("Course selection interrupted.".to_string()),
    };

    let course_config = projects_config.courses.get(&chosen_course).unwrap();

    let mut exercise_list: Vec<String> = Vec::new();

    for exercise in &course_config.exercises {
        exercise_list.push(exercise.0.clone());
    }

    if exercise_list.is_empty() {
        return Err(format!(
            "No exercises found from chosen course folder. Project directory set to {}",
            get_projects_dir().to_str().unwrap().to_string()
        ));
    }

    let chosen_exercise = match interactive_list("Select exercise: ", exercise_list) {
        Some(selection) => selection,
        None => return Err("Exercise selection interrupted.".to_string()),
    };

    let mut path = get_projects_dir();
    path.push(chosen_course);
    path.push(chosen_exercise);

    Ok(path)
}

/// Shows two interactive selections, for course and then exercise
/// Mutates parameters according to selection, giving exercise name, exercise dir and path of course_config
/// Returns error if menu was exited without selecting, if no courses/exercises were found, or if
/// config file was not read successfully
pub fn ask_exercise_interactive(
    exercise_name: &mut String,
    exercise_dir: &mut PathBuf,
    course_config: &mut Option<CourseConfig>,
) -> Result<(), String> {
    let mut exercise_path = match choose_exercise() {
        Ok(path) => path,
        Err(msg) => return Err(msg),
    };
    *exercise_name = exercise_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    *exercise_dir = exercise_path.clone();
    exercise_path.pop();
    exercise_path.push(CONFIG_FILE_NAME);
    *course_config = match read_new_course_config(&exercise_path) {
        Ok(config) => config,
        Err(msg) => return Err(msg),
    };
    Ok(())
}

/// Returns a manual progress bar of size 'length' based on percentage of 'completed' / 'total'
pub fn get_progress_string(completed: usize, total: usize, length: usize) -> String {
    let completed_proportion = if total == 0 {
        1_f32
    } else {
        completed as f32 / total as f32
    };
    let completed_percentage_readable = (completed_proportion * 100_f32).floor() as usize;
    let progress_done = (completed_proportion * length as f32).floor() as usize;

    let mut progress_string = String::with_capacity(length);
    for _ in 0..progress_done {
        progress_string.push('█');
    }
    for _ in progress_done..length {
        progress_string.push('░');
    }

    let spaces = if completed_percentage_readable < 10 {
        "   "
    } else if completed_percentage_readable < 100 {
        "  "
    } else {
        " "
    };
    format!(
        "{}{}%[{}]",
        spaces, completed_percentage_readable, progress_string
    )
}
