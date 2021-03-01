use super::command_util::Client;
use super::organization_command::set_organization;
use crate::io_module::Io;

pub fn login(io: &mut dyn Io, client: &mut dyn Client) {
    if let Ok(()) = client.load_login() {
        io.println("You are already logged in.");
        return;
    };

    io.print("Email / username: ");
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!");
        return;
    }

    io.print("Password: ");
    let mut password = io.read_password();
    password = password.trim().to_string();

    io.println("");

    match client.try_login(username, password) {
        Ok(message) => {
            io.println(&message);
            if let Err(_err) = set_organization(io, client) {
                io.println("Could not set organization");
            }
        }
        Err(message) => io.println(&message),
    }
}

#[cfg(test)]
mod tests {
    use super::super::command_util::*;
    use super::*;
    use std::path::PathBuf;
    use std::slice::Iter;
    use tmc_client::{ClientError, CourseExercise};

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    impl IoTest<'_> {
        pub fn buffer_length(&mut self) -> usize {
            self.list.len()
        }

        pub fn buffer_get(&mut self, index: usize) -> String {
            self.list[index].to_string()
        }
    }

    impl Io for IoTest<'_> {
        fn read_line(&mut self) -> String {
            match self.input.next() {
                Some(string) => string,
                None => "",
            }
            .to_string()
        }

        fn print(&mut self, output: &str) {
            print!("{}", output);
            self.list.push(output.to_string());
        }

        fn println(&mut self, output: &str) {
            println!("{}", output);
            self.list.push(output.to_string());
        }

        fn read_password(&mut self) -> String {
            self.read_line()
        }
    }

    pub struct ClientTest {}

    impl ClientTest {}

    impl Client for ClientTest {
        fn load_login(&mut self) -> Result<(), String> {
            Err("Not logged in".to_string())
        }
        fn try_login(&mut self, _username: String, _password: String) -> Result<String, String> {
            Ok("Logged in successfully!".to_string())
        }
        fn list_courses(&mut self) -> Result<Vec<Course>, String> {
            Ok(vec![
                Course {
                    name: "Kurssi1".to_string(),
                    id: 101,
                },
                Course {
                    name: "Kurssi2".to_string(),
                    id: 102,
                },
            ])
        }
        fn get_organizations(&mut self) -> Result<Vec<Organization>, String> {
            Ok(vec![
                Organization {
                    name: "Organisaatio 1".to_string(),
                    slug: "Org 1".to_string(),
                },
                Organization {
                    name: "Organisaatio 2".to_string(),
                    slug: "Org 2".to_string(),
                },
            ])
        }
        fn logout(&mut self) {}
        fn get_course_exercises(
            &mut self,
            _course_id: usize,
        ) -> Result<Vec<CourseExercise>, String> {
            // let mut exercise_list: Vec<CourseExercise> = Vec::new();
            // exercise_list.push(CourseExercise {
            //     id: 1010,
            //     available_points: Vec<ExercisePoint>,
            //     awarded_points: Vec<String>,
            //     name: "Harjoitus 1".to_string(),
            //     publish_time: Option<String>,
            //     solution_visible_after: Option<String>,
            //     deadline: Option<String>,
            //     soft_deadline: Option<String>,
            //     disabled: bool,
            //     unlocked: bool,
            // });
            // exercise_list.push(CourseExercise {
            //     name: "Harjoitus 2".to_string(),
            //     id: 1020,
            // });
            // Ok(exercise_list)
            Ok(vec![])
        }
        fn download_or_update_exercises(
            &mut self,
            _download_params: Vec<(usize, PathBuf)>,
        ) -> Result<(), ClientError> {
            Ok(())
        }
    }

    #[test]
    fn empty_username_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec!["moi"];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut client = ClientTest {};

        assert!(io.read_line().eq("moi"));

        login(&mut io, &mut client);

        assert_eq!(2, io.buffer_length());

        if io.buffer_length() == 2 {
            assert!(io
                .buffer_get(1)
                .to_string()
                .eq(&"Username cannot be empty!".to_string()));
        }
    }
}
