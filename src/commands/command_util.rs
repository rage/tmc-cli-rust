use crate::config::{ConfigValue, Credentials, TmcConfig};
use isolang::Language;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use tmc_client::{
    ClientError, Course, CourseDetails, CourseExercise, Exercise, NewSubmission, Organization,
    TmcClient, Token, SubmissionFinished
};

pub const PLUGIN: &str = "vscode_plugin";
pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct ClientProduction {
    pub tmc_client: TmcClient,
    pub test_mode: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseDetailsWrapper {
    pub unlockables: Vec<String>,
    pub exercises: Vec<Exercise>,
    pub id: usize,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub details_url: String,
    pub unlock_url: String,
    pub reviews_url: String,
    pub comet_url: String,
    pub spyware_urls: Vec<String>,
}

impl CourseDetailsWrapper {
    pub fn new(cd: CourseDetails) -> CourseDetailsWrapper {
        CourseDetailsWrapper {
            unlockables: cd.unlockables,
            exercises: cd.exercises,
            id: cd.course.id,
            name: cd.course.name,
            title: cd.course.title,
            description: cd.course.description,
            details_url: cd.course.details_url,
            unlock_url: cd.course.unlock_url,
            reviews_url: cd.course.reviews_url,
            comet_url: cd.course.comet_url,
            spyware_urls: cd.course.spyware_urls,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseConfig {
    pub username: String,
    pub server_address: String,
    pub course: CourseDetailsWrapper,
    pub organization: Organization,
    pub local_completed_exercises: Vec<String>,
    pub properties: Vec<String>,
}

/// Loads course information from file
pub fn load_course_config(path: &Path) -> Result<CourseConfig, Box<dyn Error>> {
    // TODO: errorhandling

    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

/// Saves course information to file
pub fn save_course_information(course_config: CourseConfig, pathbuf: PathBuf) {
    // TODO: errorhandling
    let f = File::create(pathbuf).expect("Unable to create file");
    let bw = BufWriter::new(f);
    serde_json::to_writer(bw, &course_config).expect("Failed writing :(");
}

use mockall::predicate::*;
use mockall::*;
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
    fn download_or_update_exercises(
        &mut self,
        download_params: Vec<(usize, PathBuf)>,
    ) -> Result<(), ClientError>;
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

impl ClientProduction {
    pub fn new(test_mode: bool) -> Self {
        let tmc_client = TmcClient::new(
            PathBuf::from("./config"),
            "https://tmc.mooc.fi".to_string(),
            PLUGIN.to_string(),
            "1.0.0".to_string(),
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
            Err(_client_error) => {
                Err("Received ClientError when calling paste command from tmc_client".to_string())
            }
            Ok(submission) => Ok(submission),
        }
    }

    fn is_test_mode(&mut self) -> bool {
        self.test_mode
    }
    fn load_login(&mut self) -> Result<(), String> {
        if self.test_mode {
            // Test login exists if config-file has key-value pair test_login = "test_logged_in"
            let config = TmcConfig::load(PLUGIN).unwrap();
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
                let mut config = TmcConfig::load(PLUGIN).unwrap();

                if let Err(_err) = config.insert(
                    "test_login".to_string(),
                    toml::Value::String("test_logged_in".to_string()),
                ) {
                    return Err("Test login value could not be changed in config file".to_string());
                }

                if let Err(_err) = config.save() {
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
                    org_list.push(Organization {
                        name: org.name,
                        slug: org.slug,

                        information: "".to_string(),
                        logo_path: "".to_string(),
                        pinned: false,
                    });
                }
                Ok(org_list)
            }
            _ => Err("Could not get organizations from server".to_string()),
        }
    }

    fn logout(&mut self) {
        if self.test_mode {
            // Remove test login from config file
            let mut config = match TmcConfig::load(PLUGIN) {
                Ok(config) => config,
                _ => panic!("Could not load the config"),
            };
            if let Err(_err) = config.remove("test_login") {
                panic!("Could not remove test login from config in test mode");
            }
            if let Err(_err) = config.save() {
                panic!("Could not save config after removing test login in test mode");
            }
            return;
        }

        let credentials = get_credentials().unwrap();

        credentials.remove().unwrap();
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

    fn download_or_update_exercises(
        &mut self,
        download_params: Vec<(usize, PathBuf)>,
    ) -> Result<(), ClientError> {
        if self.test_mode {
            return Ok(());
        }
        self.tmc_client
            .download_or_update_exercises(download_params)
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
    match TmcConfig::load(PLUGIN) {
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
    let mut config = match TmcConfig::load(PLUGIN) {
        Ok(config) => config,
        _ => return Err("Config could not be loaded".to_string()),
    };

    if let Err(_err) = config.insert(
        "organization".to_string(),
        toml::Value::String(org.to_string()),
    ) {
        return Err("Organization could not be changed".to_string());
    }

    if let Err(_err) = config.save() {
        return Err("Problem saving configurations".to_string());
    }
    Ok(())
}

pub fn get_course_id_by_name(client: &mut dyn Client, course_name: String) -> Option<usize> {
    match client.list_courses() {
        Ok(courses) => {
            for course in courses {
                if course.name == course_name {
                    return Some(course.id);
                }
            }
            None
        }
        //Err(ClientError::NotLoggedIn) => /* TODO: pass this information to caller */,
        _ => None,
    }
}

pub fn get_exercise_id_by_name(
    client: &mut dyn Client,
    course_id: usize,
    exercise_name: String,
) -> Option<usize> {
    // Notice: This doesn't return exercise.id, but instead just the index of exercise inside the course!
    match client.get_course_exercises(course_id) {
        Ok(exercises) => {
            for (exercise_id, exercise) in exercises.into_iter().enumerate() {
                if exercise.name == exercise_name {
                    return Some(exercise_id);
                }
            }
            None
        }
        //Err(ClientError::NotLoggedIn) => /* TODO: pass this information to caller */,
        _ => None,
    }
}
