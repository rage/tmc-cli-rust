use crate::config::{Config, Credentials};
use std::path::PathBuf;
use tmc_client::{TmcClient, ClientError, CourseExercise, Token};


pub const PLUGIN: &str = "vscode_plugin";
pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct Organization {
    pub name: String,
    pub slug: String,
}

pub struct Client {
    pub tmc_client: TmcClient,
    pub test_mode: bool,
}

pub struct Course {
    pub name: String,
    pub id: usize,
}

impl Client {
    pub fn new(test_mode: bool) -> Self {
        let tmc_client = TmcClient::new(
                PathBuf::from("./config"),
                "https://tmc.mooc.fi".to_string(),
                PLUGIN.to_string(),
                "1.0.0".to_string(),
            )
            .unwrap();
        Client {
            tmc_client,
            test_mode,
        }
    }

    pub fn load_login(&mut self) -> Result<(), String> {
        if let Some(credentials) = get_credentials() {
            match self.tmc_client.set_token(credentials.token()) {
                Ok(()) => return Ok(()),
                _ => return Err("Setting login token failed".to_string()),
            }
        }
        Err("No login found. You need to be logged in to use this command".to_string())
    }

    pub fn try_login(&mut self, username: String, password: String) -> Result<String, String> {

        let token;
    
        if self.test_mode {
            if username == "testusername" && password == "testpassword" {
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

    fn authenticate(&mut self, username: String, password: String) -> Result<Token, String> {

        match self.tmc_client.authenticate(PLUGIN, username, password) {
            Ok(x) => Ok(x),
            Err(x) => Err(Client::explain_login_fail(x)),
        }
    }
    
    fn explain_login_fail(error: ClientError) -> String {
        let res = format!("{:?}", error);
    
        if res.contains("The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client.") {
            return WRONG_LOGIN.to_string();
        }
    
        "Login failed with an unknown error message".to_string()
    }

    pub fn list_courses(&mut self) -> Result<Vec<Course>, String> {
        if self.test_mode {
            // TODO return a mock list of courses for test mode
            return Err("Test feature not yet implemented".to_string());
        }
        match self.tmc_client.list_courses(&get_organization().unwrap()) {
            Ok(courses) => {
                let mut course_list: Vec<Course> = Vec::new();
                for course in courses {
                    course_list.push(Course{name: course.name, id: course.id});
                }
                Ok(course_list)
            },
            Err(ClientError::NotLoggedIn) => {
                Err("Login token is invalid. Please try logging in again.".to_string())
            },
            _ => Err("Unknown error. Please try again.".to_string()),
        }
    }

    pub fn get_organizations(&mut self) -> Result<Vec<Organization>, String> {
        if self.test_mode {
            return Err("Test feature not yet implemented".to_string());
        }
        let result = self.tmc_client.get_organizations();
        match result {
            Ok(organizations) => {
                let mut org_list: Vec<Organization> = Vec::new();
                for org in organizations {
                    org_list.push(Organization { name: org.name, slug: org.slug});
                }
                Ok(org_list)
            },
            _ => Err("Could not get organizations from server".to_string()),
        }
    }

    pub fn logout(&mut self) {
        let credentials = get_credentials().unwrap();

        credentials.remove().unwrap();
    }



    pub fn get_course_exercises(&mut self, course_id: usize) -> Result<Vec<CourseExercise>, String> {
        match self.tmc_client.get_course_exercises(course_id) {
            Ok(exercises) => Ok(exercises),
            Err(ClientError::NotLoggedIn) => Err("Login token is invalid. Please try logging in again.".to_string()),
            _ => Err("Unknown error. Please try again.".to_string()),
        }
    }

    pub fn download_or_update_exercises(&mut self, download_params: Vec<(usize, PathBuf)>) -> Result<String, String> {
        if self.test_mode {
            return Err("Test feature not yet implemented".to_string());
        }
        match self.tmc_client.download_or_update_exercises(download_params) {
            Ok(()) => Ok("Downloaded exercises successfully".to_string()),
            _ => Err("Downloading exercises failed".to_string()),
        }
    }
    
}



pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap()
}

// Returns slug of organization as String (if successful)
#[allow(dead_code)]
pub fn get_organization() -> Option<String> {
    let config = Config::load(PLUGIN).unwrap();

    Some(config.get_value("organization").unwrap())
}

pub fn set_organization(org: &str) -> Result<(), &'static str> {
    let mut config = Config::new(PLUGIN);

    if let Err(_err) = config.change_value("organization", org) {
        return Err("Organization could not be changed");
    }

    if let Err(_err) = config.save() {
        return Err("Problem saving configurations");
    }
    Ok(())
}

pub fn get_course_id_by_name(client: &mut Client, course_name: String) -> Option<usize> {
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
