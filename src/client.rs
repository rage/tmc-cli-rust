use crate::{commands::util, config::TmcCliConfig, PLUGIN, PLUGIN_VERSION};
use anyhow::Context;
use reqwest::Url;
use std::path::Path;
use tmc_langs::{
    tmc::{
        response::{
            Course, CourseDetails, CourseExercise, NewSubmission, Organization, SubmissionFinished,
        },
        TestMyCodeClient, TestMyCodeClientError, Token,
    },
    Credentials, DownloadOrUpdateCourseExercisesResult, DownloadResult, LangsError, Language,
};

pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct Client {
    pub tmc_client: TestMyCodeClient,
    pub test_mode: bool,
}

impl Client {
    pub fn new(tmc_root_url: Url, test_mode: bool) -> anyhow::Result<Self> {
        let (tmc_client, _credentials) = tmc_langs::init_testmycode_client_with_credentials(
            tmc_root_url,
            PLUGIN,
            PLUGIN_VERSION,
        )?;

        Ok(Client {
            tmc_client,
            test_mode,
        })
    }

    pub fn is_test_mode(&mut self) -> bool {
        self.test_mode
    }

    pub fn authenticate(&mut self, username: String, password: String) -> anyhow::Result<Token> {
        // match self.tmc_client.authenticate(PLUGIN, username, password) {
        match tmc_langs::login_with_password(&mut self.tmc_client, PLUGIN, username, password) {
            Ok(x) => Ok(x),
            Err(x) => anyhow::bail!(Client::explain_login_fail(x)),
        }
    }

    pub fn explain_login_fail(error: LangsError) -> String {
        let res = format!("{error:?}");

        if res.contains("The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client.") {
            return WRONG_LOGIN.to_string();
        }

        "Login failed with an unknown error message".to_string()
    }

    // tmc commands
    pub fn paste(
        &self,
        projects_dir: &Path,
        course_slug: &str,
        exercise_slug: &str,
        paste_message: Option<String>,
        locale: Option<Language>,
    ) -> Result<NewSubmission, String> {
        if self.test_mode {
            return Err("Integration test input not yet implemented for paste command".to_string());
        }
        match tmc_langs::paste_exercise(
            &self.tmc_client,
            projects_dir,
            course_slug,
            exercise_slug,
            paste_message,
            locale,
        ) {
            Err(client_error) => match client_error {
                LangsError::TestMyCodeClient(TestMyCodeClientError::HttpError { status, error, .. }) => {
                    Err(format!("Status {status}, message: {error}"))
                }
                _ => Err(
                    "Received unhandled TestMyCodeClientError when calling paste command from tmc_client"
                        .to_string(),
                ),
            },
            Ok(submission) => Ok(submission),
        }
    }

    pub fn load_login(&mut self, config: &TmcCliConfig) -> anyhow::Result<()> {
        if self.test_mode {
            // Test login exists if config-file has key-value pair test_login = "test_logged_in"
            let test_login_exists = config
                .get_test_login()
                .map(|tl| tl == "test_logged_in")
                .unwrap_or_default();
            if test_login_exists {
                return Ok(());
            } else {
                anyhow::bail!("No login found".to_string());
            }
        }

        if let Some(credentials) = util::get_credentials() {
            self.tmc_client.set_token(credentials.token());
            Ok(())
        } else {
            anyhow::bail!("No login found. You need to be logged in to use this command");
        }
    }

    pub fn try_login(
        &mut self,
        username: String,
        password: String,
        config: &mut TmcCliConfig,
    ) -> anyhow::Result<String> {
        if self.test_mode {
            if username == "testusername" && password == "testpassword" {
                config.set_test_login();

                if let Err(_err) = config.save() {
                    anyhow::bail!("Problem saving login");
                }

                return Ok(SUCCESSFUL_LOGIN.to_string());
            }
            anyhow::bail!(WRONG_LOGIN);
        }

        let token = self.authenticate(username, password)?;
        if Credentials::save(PLUGIN, token).is_ok() {
            return Ok(SUCCESSFUL_LOGIN.to_string());
        };

        anyhow::bail!("Error! Saving credentials failed")
    }

    pub fn list_courses(&mut self, org: &str) -> anyhow::Result<Vec<Course>> {
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

        match self.tmc_client.list_courses(org) {
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
            Err(TestMyCodeClientError::NotAuthenticated) => {
                anyhow::bail!("Login token is invalid. Please try logging in again.")
            }
            Err(err) => anyhow::bail!("Unexpected error: '{err}'."),
        }
    }

    pub fn get_organizations(&mut self) -> anyhow::Result<Vec<Organization>> {
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
        let organizations = self
            .tmc_client
            .get_organizations()
            .context("Could not get organizations from server")?;
        Ok(organizations)
    }

    pub fn logout(&mut self, config: &mut TmcCliConfig) -> anyhow::Result<()> {
        if self.test_mode {
            // Remove test login from config file
            config.remove_test_login();
            config
                .save()
                .context("Could not save config after removing test login in test mode")?;
            return Ok(());
        }

        let credentials = util::get_credentials().context("Failed to get credentials")?;
        credentials.remove()?;
        Ok(())
    }

    pub fn wait_for_submission(
        &self,
        submission_url: Url,
    ) -> Result<SubmissionFinished, TestMyCodeClientError> {
        self.tmc_client.wait_for_submission_at(submission_url)
    }
    pub fn update_exercises(
        &mut self,
        path: &Path,
    ) -> Result<DownloadOrUpdateCourseExercisesResult, LangsError> {
        if self.test_mode {
            return Ok(DownloadOrUpdateCourseExercisesResult {
                downloaded: vec![],
                skipped: vec![],
                failed: None,
            });
        }

        tmc_langs::update_exercises(&self.tmc_client, path)
    }

    pub fn submit(
        &self,
        projects_dir: &Path,
        course_slug: &str,
        exercise_slug: &str,
        locale: Option<Language>,
    ) -> Result<NewSubmission, LangsError> {
        if self.test_mode {
            return Ok(NewSubmission {
                show_submission_url: "https://tmc.mooc.fi/submissions/7400888".to_string(),
                paste_url: "url".to_string(),
                submission_url: "https://tmc.mooc.fi/api/v8/core/submissions/7400888".to_string(),
            });
        }
        tmc_langs::submit_exercise(
            &self.tmc_client,
            projects_dir,
            course_slug,
            exercise_slug,
            locale,
        )
    }

    pub fn get_course_exercises(&mut self, course_id: u32) -> anyhow::Result<Vec<CourseExercise>> {
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
            Err(TestMyCodeClientError::NotAuthenticated) => {
                anyhow::bail!("Login token is invalid. Please try logging in again.")
            }
            Err(err) => anyhow::bail!("Unexpected error: '{err}'."),
        }
    }

    pub fn download_or_update_exercises(
        &mut self,
        exercise_ids: &[u32],
        projects_dir: &Path,
    ) -> Result<DownloadResult, LangsError> {
        if self.test_mode {
            return Ok(DownloadResult::Success {
                downloaded: vec![],
                skipped: vec![],
            });
        }

        tmc_langs::download_or_update_course_exercises(
            &self.tmc_client,
            projects_dir,
            exercise_ids,
            true,
        )
    }

    pub fn get_course_details(
        &self,
        course_id: u32,
    ) -> Result<CourseDetails, TestMyCodeClientError> {
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

    #[cfg(test)]
    pub fn set_tmc_token(&mut self, token: Token) {
        self.tmc_client.set_token(token);
    }
}
