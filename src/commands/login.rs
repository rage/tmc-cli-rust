use super::{download, organization, util, util::Client};
use crate::io::{Io, PrintColor};
use anyhow::Context;

pub fn login(
    io: &mut dyn Io,
    client: &mut dyn Client,
    interactive_mode: bool,
) -> anyhow::Result<()> {
    io.print("Email / username: ", PrintColor::Normal)?;
    let mut username = io.read_line()?;
    username = username.trim().to_string();

    if username.is_empty() {
        anyhow::bail!("Username cannot be empty!");
    }

    io.print("Password: ", PrintColor::Normal)?;

    // Read password without rpassword if ran in --testmode, because rpassword
    // is not able to read mock stdin input in binary tests
    let mut password = if client.is_test_mode() {
        io.read_line()?
    } else {
        io.read_password()?
    };
    password = password.trim().to_string();

    let message = client.try_login(username, password)?;
    io.println(&message, PrintColor::Success)?;

    if interactive_mode {
        organization::set_organization(io, client)
    } else {
        organization::set_organization_old(io, client)
    }
    .context("Could not set organization")?;

    if client.is_test_mode() {
        return Ok(());
    }

    if interactive_mode {
        download_after_login(client, io)?;
    } else {
        io.println("Logged in and selected organization", PrintColor::Success)?;
    }
    Ok(())
}

pub fn download_after_login(client: &mut dyn Client, io: &mut dyn Io) -> anyhow::Result<()> {
    io.println("Fetching courses...", PrintColor::Normal)?;
    let courses = client.list_courses()?;

    let mut courses = courses
        .iter()
        .map(|course| client.get_course_details(course.id))
        .collect::<Result<Vec<_>, _>>()?;

    courses.sort_by(|a, b| {
        a.course
            .title
            .to_lowercase()
            .cmp(&b.course.title.to_lowercase())
    });

    let mut courses_displayed = courses
        .iter()
        .map(|course| course.course.title.as_str())
        .collect::<Vec<_>>();
    let no_download = "Don't download anything".to_string();
    courses_displayed.insert(0, no_download.as_str());

    let course = util::get_course_name(&courses_displayed)?;
    if course == no_download {
        anyhow::bail!("No course downloaded.");
    }
    let name_select = &courses
        .iter()
        .find(|c| c.course.title == course)
        .context("No course matching the selected name was found")?
        .course
        .name;

    // Get course by name
    let course = util::get_course_by_name(client, name_select)?
        .ok_or_else(|| anyhow::anyhow!("Could not find course with that name"))?;
    let path = util::get_projects_dir()?;

    let msg = download::download_exercises(&path, client, &course)?;
    io.println(&msg, PrintColor::Success)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice::Iter;

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    impl Io for IoTest<'_> {
        fn read_line(&mut self) -> anyhow::Result<String> {
            let res = match self.input.next() {
                Some(string) => string,
                None => "",
            };
            Ok(res.to_string())
        }

        fn print(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            print!("{output}");
            self.list.push(output.to_string());
            Ok(())
        }

        fn println(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            println!("{output}");
            self.list.push(output.to_string());
            Ok(())
        }

        fn read_password(&mut self) -> anyhow::Result<String> {
            self.read_line()
        }
    }

    /*
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
    } */
}
