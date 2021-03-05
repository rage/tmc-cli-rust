use crate::config::{ConfigValue, Credentials, TmcConfig};
use std::path::PathBuf;
use tmc_client::{ClientError, CourseExercise, TmcClient, Token};

pub const PLUGIN: &str = "vscode_plugin";
pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct Organization {
    pub name: String,
    pub slug: String,
}

pub struct ClientProduction {
    pub tmc_client: TmcClient,
    pub test_mode: bool,
}

pub struct Course {
    pub name: String,
    pub id: usize,
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
    fn get_course_exercises(&mut self, course_id: usize) -> Result<Vec<CourseExercise>, String>;
    fn download_or_update_exercises(
        &mut self,
        download_params: Vec<(usize, PathBuf)>,
    ) -> Result<(), ClientError>;
    fn is_test_mode(&mut self) -> bool;
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
                },
                Course {
                    name: "imaginary-test-course".to_string(),
                    id: 1,
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
                },
                Organization {
                    name: "imaginary test organization".to_string(),
                    slug: "imag".to_string(),
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
            let mut config = TmcConfig::load(PLUGIN).unwrap();
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
}

pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap()
}

// Returns slug of organization as String (if successful)
#[allow(dead_code)]
pub fn get_organization() -> Option<String> {
    let config = TmcConfig::load(PLUGIN).unwrap();

    // convert the toml::Value to String (if possible)
    match config.get("organization") {
        ConfigValue::Value(Some(value)) => Some(toml::Value::as_str(&value).unwrap().to_string()),
        _ => None,
    }
}

pub fn set_organization(org: &str) -> Result<(), String> {
    let mut config = TmcConfig::load(PLUGIN).unwrap();

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
