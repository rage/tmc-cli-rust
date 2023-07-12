use super::{download, util, util::Client};
use crate::io::{Io, PrintColor};
use anyhow::Context;

pub fn login(io: &mut Io, client: &mut Client, _interactive_mode: bool) -> anyhow::Result<()> {
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

    /*
    now that courses mooc is supported,
    no need to always select an org
    if interactive_mode {
        organization::set_organization(io, client)
    } else {
        organization::set_organization_old(io, client)
    }
    .context("Could not set organization")?;
    */

    if client.is_test_mode() {
        return Ok(());
    }

    /*
    now that courses mooc is supported,
    no need to always select a course
    if interactive_mode {
        download_after_login(client, io)?;
    } else {

    }
    */

    io.println("Logged in", PrintColor::Success)?;
    Ok(())
}

pub fn _download_after_login(client: &mut Client, io: &mut Io) -> anyhow::Result<()> {
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
