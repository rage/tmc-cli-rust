use crate::interactive::interactive_list;
use anyhow::Context;
use isolang::Language;
use reqwest::Url;
use std::{
    env,
    path::{Path, PathBuf},
};
use tmc_langs::{
    ClientError, ConfigValue, Course, CourseDetails, CourseExercise, Credentials,
    DownloadOrUpdateCourseExercisesResult, DownloadResult, ExercisesDetails, LangsError,
    NewSubmission, Organization, ProjectsConfig, SubmissionFinished, TmcClient, TmcConfig, Token,
};

pub const PLUGIN: &str = "tmc_cli_rust";
pub const PLUGIN_VERSION: &str = "1.0.5";
pub const SUCCESSFUL_LOGIN: &str = "Logged in successfully!";
pub const WRONG_LOGIN: &str = "Wrong username or password";

pub struct ClientProduction {
    pub tmc_client: TmcClient,
    pub test_mode: bool,
}

pub trait Client {
    fn load_login(&mut self) -> anyhow::Result<()>;
    fn try_login(&mut self, username: String, password: String) -> anyhow::Result<String>;
    fn list_courses(&mut self) -> anyhow::Result<Vec<Course>>;
    fn get_organizations(&mut self) -> anyhow::Result<Vec<Organization>>;
    fn logout(&mut self) -> anyhow::Result<()>;
    fn wait_for_submission(&self, submission_url: Url) -> Result<SubmissionFinished, ClientError>;
    fn submit(
        &self,
        projects_dir: &Path,
        course_slug: &str,
        exercise_slug: &str,
        locale: Option<Language>,
    ) -> Result<NewSubmission, LangsError>;
    fn get_course_exercises(&mut self, course_id: u32) -> anyhow::Result<Vec<CourseExercise>>;
    fn get_exercise_details(
        &mut self,
        exercise_ids: Vec<u32>,
    ) -> Result<Vec<ExercisesDetails>, ClientError>;
    fn download_or_update_exercises(
        &mut self,
        download_params: &[u32],
        path: &Path,
    ) -> Result<DownloadResult, LangsError>;
    fn is_test_mode(&mut self) -> bool;
    fn get_course_details(&self, course_id: u32) -> Result<CourseDetails, ClientError>;
    fn get_organization(&self, organization_slug: &str) -> Result<Organization, ClientError>;
    fn update_exercises(
        &mut self,
        path: &Path,
    ) -> Result<DownloadOrUpdateCourseExercisesResult, LangsError>;
    fn paste(
        &self,
        projects_dir: &Path,
        course_slug: &str,
        exercise_slug: &str,
        paste_message: Option<String>,
        locale: Option<Language>,
    ) -> Result<NewSubmission, String>;
}

impl ClientProduction {
    pub fn new(test_mode: bool) -> anyhow::Result<Self> {
        let (tmc_client, _credentials) = tmc_langs::init_tmc_client_with_credentials(
            Url::parse("https://tmc.mooc.fi").expect("Server address should always be correct."),
            PLUGIN,
            PLUGIN_VERSION,
        )?;

        Ok(ClientProduction {
            tmc_client,
            test_mode,
        })
    }
    #[allow(dead_code)]
    pub fn is_test_mode(&mut self) -> bool {
        self.test_mode
    }

    fn authenticate(&mut self, username: String, password: String) -> anyhow::Result<Token> {
        // match self.tmc_client.authenticate(PLUGIN, username, password) {
        match tmc_langs::login_with_password(&mut self.tmc_client, PLUGIN, username, password) {
            Ok(x) => Ok(x),
            Err(x) => anyhow::bail!(ClientProduction::explain_login_fail(x)),
        }
    }

    fn explain_login_fail(error: LangsError) -> String {
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
                LangsError::TmcClient(ClientError::HttpError {
                    url: _,
                    status,
                    error,
                    obsolete_client: _,
                }) => Err(format!("Status {}, message: {}", status, error)),
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

    fn load_login(&mut self) -> anyhow::Result<()> {
        if self.test_mode {
            // Test login exists if config-file has key-value pair test_login = "test_logged_in"
            let config = TmcConfig::load(PLUGIN, &get_path()?)?;
            let test_login_exists = match config.get("test_login") {
                ConfigValue::Value(Some(value)) => {
                    toml::Value::as_str(&value).context("invalid value")? == "test_logged_in"
                }
                _ => false,
            };
            if test_login_exists {
                return Ok(());
            } else {
                anyhow::bail!("No login found".to_string());
            }
        }

        if let Some(credentials) = get_credentials() {
            self.tmc_client.set_token(credentials.token());
            Ok(())
        } else {
            anyhow::bail!("No login found. You need to be logged in to use this command");
        }
    }

    fn try_login(&mut self, username: String, password: String) -> anyhow::Result<String> {
        if self.test_mode {
            if username == "testusername" && password == "testpassword" {
                let mut config = TmcConfig::load(PLUGIN, &get_path()?)?;

                if let Err(_err) = config.insert(
                    "test_login".to_string(),
                    toml::Value::String("test_logged_in".to_string()),
                ) {
                    anyhow::bail!("Test login value could not be changed in config file");
                }

                if let Err(_err) = config.save(&get_path()?) {
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

    fn list_courses(&mut self) -> anyhow::Result<Vec<Course>> {
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

        match self.tmc_client.list_courses(&get_organization()?) {
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
            Err(ClientError::NotAuthenticated) => {
                anyhow::bail!("Login token is invalid. Please try logging in again.")
            }
            _ => anyhow::bail!("Unknown error. Please try again."),
        }
    }

    fn get_organizations(&mut self) -> anyhow::Result<Vec<Organization>> {
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
            _ => anyhow::bail!("Could not get organizations from server"),
        }
    }

    fn logout(&mut self) -> anyhow::Result<()> {
        if self.test_mode {
            // Remove test login from config file
            let mut config =
                TmcConfig::load(PLUGIN, &get_path()?).context("Could not load the config")?;
            config
                .remove("test_login")
                .context("Could not remove test login from config in test mode")?;
            config
                .save(&get_path()?)
                .context("Could not save config after removing test login in test mode")?;
            return Ok(());
        }

        let credentials = get_credentials().context("Failed to get credentials")?;
        credentials.remove()?;
        Ok(())
    }

    fn wait_for_submission(&self, submission_url: Url) -> Result<SubmissionFinished, ClientError> {
        self.tmc_client.wait_for_submission_at(submission_url)
    }
    fn update_exercises(
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

    fn submit(
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

    fn get_course_exercises(&mut self, course_id: u32) -> anyhow::Result<Vec<CourseExercise>> {
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
            Err(ClientError::NotAuthenticated) => {
                anyhow::bail!("Login token is invalid. Please try logging in again.")
            }
            _ => anyhow::bail!("Unknown error. Please try again."),
        }
    }

    fn get_exercise_details(
        &mut self,
        exercise_ids: Vec<u32>,
    ) -> Result<Vec<ExercisesDetails>, ClientError> {
        if self.test_mode {
            return Ok(vec![ExercisesDetails {
                id: 0,
                course_name: "test_course".to_string(),
                exercise_name: "test_exercise".to_string(),
                checksum: "test_checksum".to_string(),
                hide_submission_results: false,
            }]);
        }
        self.tmc_client.get_exercises_details(&exercise_ids)
    }

    fn download_or_update_exercises(
        &mut self,
        exercise_ids: &[u32],
        path: &Path,
    ) -> Result<DownloadResult, LangsError> {
        if self.test_mode {
            return Ok(DownloadResult::Success {
                downloaded: vec![],
                skipped: vec![],
            });
        }

        tmc_langs::download_or_update_course_exercises(&self.tmc_client, path, exercise_ids, true)
    }

    fn get_course_details(&self, course_id: u32) -> Result<CourseDetails, ClientError> {
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
    fn get_organization(&self, organization_slug: &str) -> Result<Organization, ClientError> {
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
pub fn get_organization() -> anyhow::Result<String> {
    let config = TmcConfig::load(PLUGIN, &get_path()?)?;
    // convert the toml::Value to String (if possible)
    match config.get("organization") {
        ConfigValue::Value(Some(value)) => Ok(toml::Value::as_str(&value)
            .context("invalid value")?
            .to_string()),
        _ => anyhow::bail!("missing value"),
    }
}

pub fn set_organization(org: &str) -> anyhow::Result<()> {
    let mut config = match TmcConfig::load(PLUGIN, &get_path()?) {
        Ok(config) => config,
        _ => anyhow::bail!("Config could not be loaded"),
    };

    config
        .insert(
            "organization".to_string(),
            toml::Value::String(org.to_string()),
        )
        .context("Organization could not be changed")?;

    config
        .save(&get_path()?)
        .context("Problem saving configurations")?;
    Ok(())
}

/// Returns course as: Ok(Some(Course)) or Ok(None) if not found, Err(msg) if could not get courses list
pub fn get_course_by_name(
    client: &mut dyn Client,
    course_name: &str,
) -> anyhow::Result<Option<Course>> {
    let courses = client.list_courses()?;
    Ok(courses.into_iter().find(|c| c.name == course_name))
}

/// Finds an exercise
/// Priority to check for valid exercise path:
/// 1. Checks optional parameter
/// 2. Checks current directory
/// 3. Checks central ProjectsConfig with interactive menu
///
/// # Errors
/// Returns an error if the last chance, interactive menu, fails.
pub fn exercise_pathfinder(path: Option<&str>) -> anyhow::Result<PathBuf> {
    // check if parameter was given
    if let Some(ex_path) = path {
        let buf = PathBuf::from(ex_path);
        if is_exercise_dir(buf.clone())? {
            return Ok(buf);
        } else {
            anyhow::bail!("Invalid exercise path given");
        }
    }

    let current_path = env::current_dir().ok();

    // check if current path is an exercise_dir,
    // in any other case use interactive menu
    match current_path {
        Some(ex_path) => match is_exercise_dir(ex_path.clone()) {
            Ok(is_ex_path) => {
                if is_ex_path {
                    Ok(ex_path)
                } else {
                    choose_exercise()
                }
            }
            Err(_err) => choose_exercise(),
        },
        None => choose_exercise(),
    }
}

pub fn get_path() -> anyhow::Result<PathBuf> {
    TmcConfig::get_location(PLUGIN).map_err(Into::into)
}

pub fn get_projects_dir() -> anyhow::Result<PathBuf> {
    tmc_langs::get_projects_dir(PLUGIN).map_err(Into::into)
}

/// Choose course and then exercise interactively, return exercise path
/// or Err(String) if either menu is interrupted or no items found
pub fn choose_exercise() -> anyhow::Result<PathBuf> {
    let mut courses: Vec<String> = Vec::new();

    let projects_config = match ProjectsConfig::load(&get_projects_dir()?) {
        Ok(projects_config) => projects_config,
        Err(_err) => anyhow::bail!("Could not load info about projects"),
    };

    for course in &projects_config.courses {
        courses.push(course.0.clone());
    }

    if courses.is_empty() {
        anyhow::bail!(
            "No courses found from current or project directory. Project directory set to {}",
            get_projects_dir()?
                .to_str()
                .context("invalid projects dir")?
        );
    }

    let chosen_course = match interactive_list("First select course: ", courses)? {
        Some(selection) => selection,
        None => anyhow::bail!("Course selection interrupted."),
    };

    let course_config = projects_config
        .courses
        .get(&chosen_course)
        .context("Failed to find selected course")?;

    let mut exercise_list: Vec<String> = Vec::new();

    for exercise in &course_config.exercises {
        exercise_list.push(exercise.0.clone());
    }

    if exercise_list.is_empty() {
        anyhow::bail!(
            "No exercises found from chosen course folder. Project directory set to {}",
            get_projects_dir()?
                .to_str()
                .context("invalid projects dir")?
        );
    }

    let chosen_exercise = match interactive_list("Select exercise: ", exercise_list)? {
        Some(selection) => selection,
        None => anyhow::bail!("Exercise selection interrupted."),
    };

    let mut path = get_projects_dir()?;
    path.push(chosen_course);
    path.push(chosen_exercise);

    Ok(path)
}

/// Parses an exercise path into (projects_dir, course_name, exercise_name)
///
/// # Errors
/// Returns an error if there was problems reading file_names
pub fn parse_exercise_dir(mut exercise_dir: PathBuf) -> anyhow::Result<(PathBuf, String, String)> {
    let exercise_slug = exercise_dir
        .file_name()
        .context("could not get exercise name")?
        .to_str()
        .context("could not get exercise name")?
        .to_string();

    exercise_dir.pop();
    let course_slug = exercise_dir
        .file_name()
        .context("could not get exercise name")?
        .to_str()
        .context("could not get exercise name")?
        .to_string();

    exercise_dir.pop();

    Ok((exercise_dir, course_slug, exercise_slug))
}

/// Checks if provided directory contains an exercise
///
/// # Errors
/// Returns an error if it failed to load ProjectsConfig
/// Or failed to read paths
pub fn is_exercise_dir(dir: PathBuf) -> anyhow::Result<bool> {
    let (projects_dir, course_slug, _exercise_slug) = parse_exercise_dir(dir)?;
    let config = ProjectsConfig::load(projects_dir.as_path()).with_context(|| {
        format!(
            "Failed to load projects config from directory '{}'",
            projects_dir.display(),
        )
    })?;

    Ok(config.courses.contains_key(&course_slug))
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
