use super::command_util::Client;
use super::{command_util, download_command, organization_command};
use crate::io_module::{Io, PrintColor};

pub fn login(io: &mut dyn Io, client: &mut dyn Client, interactive_mode: bool) {
    io.print("Email / username: ", PrintColor::Normal);
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!", PrintColor::Failed);
        return;
    }

    io.print("Password: ", PrintColor::Normal);

    let mut password;

    // Read password without rpassword if ran in --testmode, because rpassword
    // is not able to read mock stdin input in binary tests
    if client.is_test_mode() {
        password = io.read_line();
    } else {
        password = io.read_password();
    }
    password = password.trim().to_string();

    match client.try_login(username, password) {
        Ok(message) => {
            io.println(&message, PrintColor::Success);

            let res = if interactive_mode {
                organization_command::set_organization(io, client)
            } else {
                organization_command::set_organization_old(io, client)
            };
            if let Err(_err) = res {
                io.println("Could not set organization", PrintColor::Failed);
                return;
            }
        }
        Err(message) => {
            io.println(&message, PrintColor::Failed);
            return;
        }
    }

    if client.is_test_mode() {
        return;
    }

    if interactive_mode {
        download_after_login(client, io);
    } else {
        io.println("Logged in and selected organization", PrintColor::Success);
    }
}

pub fn download_after_login(client: &mut dyn Client, io: &mut dyn Io) {
    io.println("Fetching courses...", PrintColor::Normal);
    let courses = client.list_courses();
    if courses.is_err() {
        io.println("Could not list courses.", PrintColor::Failed);
        return;
    }

    let mut courses = courses
        .unwrap()
        .iter()
        .map(|course| client.get_course_details(course.id).unwrap())
        .collect::<Vec<_>>();

    courses.sort_by(|a, b| {
        a.course
            .title
            .to_lowercase()
            .cmp(&b.course.title.to_lowercase())
    });

    let mut courses_displayed = courses
        .iter()
        .map(|course| course.course.title.clone())
        .collect::<Vec<_>>();
    let no_download = "Don't download anything".to_string();
    courses_displayed.insert(0, no_download.clone());

    let name_select = match download_command::get_course_name(courses_displayed) {
        Ok(course) => {
            if course == no_download {
                io.println("No course downloaded.", PrintColor::Normal);
                return;
            }
            courses
                .iter()
                .find(|c| c.course.title == course)
                .unwrap()
                .course
                .name
                .clone()
        }
        Err(msg) => {
            io.println(&msg, PrintColor::Failed);
            return;
        }
    };

    // Get course by name
    let course_result = match command_util::get_course_by_name(client, name_select) {
        Ok(result) => result,
        Err(msg) => {
            io.println(&msg, PrintColor::Failed);
            return;
        }
    };

    if course_result.is_none() {
        io.println("Could not find course with that name", PrintColor::Failed);
        return;
    }
    let course = course_result.unwrap();

    let pathbuf = command_util::get_projects_dir();

    match download_command::download_exercises(pathbuf, client, course) {
        Ok(msg) => io.println(&msg, PrintColor::Success),
        Err(msg) => io.println(&msg, PrintColor::Failed),
    }
}

#[cfg(test)]
mod tests {
    use super::super::command_util::*;
    use super::*;
    use std::slice::Iter;
    use tmc_client::Organization;

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

        fn print(&mut self, output: &str, _font_color: PrintColor) {
            print!("{}", output);
            self.list.push(output.to_string());
        }

        fn println(&mut self, output: &str, _font_color: PrintColor) {
            println!("{}", output);
            self.list.push(output.to_string());
        }

        fn read_password(&mut self) -> String {
            self.read_line()
        }
    }

    #[test]
    fn login_with_incorrect_username_or_password_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec!["test_username", "test_password"];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login().returning(|| Err("".to_string()));
        mock.expect_is_test_mode().returning(|| true);

        let _username = String::from("test_username");
        let _password = String::from("test_password");

        mock.expect_try_login()
            .returning(|_username, _password| Err("error_message".to_string()));

        login(&mut io, &mut mock, false);

        assert_eq!(3, io.buffer_length());
        if io.buffer_length() == 3 {
            assert!(io
                .buffer_get(2)
                .to_string()
                .eq(&"error_message".to_string()));
        }
    }

    #[test]
    fn login_with_correct_username_and_password_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec!["test_username", "test_password", "wrong_slug"];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login().returning(|| Err("".to_string()));
        mock.expect_is_test_mode().returning(|| true);

        let _username = String::from("test_username");
        let _password = String::from("test_password");

        mock.expect_try_login()
            .returning(|_username, _password| Ok("ok_message_for_try_login".to_string()));

        mock.expect_get_organizations().returning(|| {
            Ok(vec![
                Organization {
                    name: "org1".to_string(),
                    slug: "slug_org1".to_string(),
                    information: "".to_string(),
                    logo_path: "".to_string(),
                    pinned: false,
                },
                Organization {
                    name: "org2".to_string(),
                    slug: "slug_org2".to_string(),
                    information: "".to_string(),
                    logo_path: "".to_string(),
                    pinned: false,
                },
            ])
        });

        login(&mut io, &mut mock, false);

        assert_eq!(14, io.buffer_length());

        if io.buffer_length() == 13 {
            assert!(io
                .buffer_get(2)
                .to_string()
                .eq(&"ok_message_for_try_login".to_string()));
            assert!(io.buffer_get(8).to_string().eq(&"org2".to_string()));
            assert!(io.buffer_get(9).to_string().eq(&" Slug: ".to_string()));
            assert!(io.buffer_get(10).to_string().eq(&"slug_org2".to_string()));
            assert!(io
                .buffer_get(11)
                .to_string()
                .eq(&"\nChoose organization by writing its slug: ".to_string()));
            assert!(io
                .buffer_get(12)
                .to_string()
                .eq(&"Could not set organization".to_string()));
        }
    }
}
