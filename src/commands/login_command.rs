use super::command_util::Client;
use super::organization_command;
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
            if let Err(_err) = organization_command::set_organization(io, client) {
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
    use std::slice::Iter;

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

    #[test]
    fn already_logged_in_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec!["test_that_buffer_for_test_input_works"];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };
        assert!(io.read_line().eq("test_that_buffer_for_test_input_works"));

        let mut mock = MockClient::new();
        mock.expect_load_login().times(1).returning(|| Ok(()));

        login(&mut io, &mut mock);

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"You are already logged in.".to_string()));
        }
    }

    #[test]
    fn empty_username_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login()
            .times(1)
            .returning(|| Err("".to_string()));

        login(&mut io, &mut mock);

        assert_eq!(2, io.buffer_length());
        if io.buffer_length() == 2 {
            assert!(io
                .buffer_get(1)
                .to_string()
                .eq(&"Username cannot be empty!".to_string()));
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

        let _username = String::from("test_username");
        let _password = String::from("test_password");

        mock.expect_try_login()
            .returning(|_username, _password| Err("error_message".to_string()));

        login(&mut io, &mut mock);

        assert_eq!(4, io.buffer_length());
        if io.buffer_length() == 4 {
            assert!(io
                .buffer_get(3)
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

        let _username = String::from("test_username");
        let _password = String::from("test_password");

        mock.expect_try_login()
            .returning(|_username, _password| Ok("ok_message_for_try_login".to_string()));

        mock.expect_get_organizations().returning(|| {
            Ok(vec![
                Organization {
                    name: "org1".to_string(),
                    slug: "slug_org1".to_string(),
                },
                Organization {
                    name: "org2".to_string(),
                    slug: "slug_org2".to_string(),
                },
            ])
        });

        login(&mut io, &mut mock);

        assert_eq!(12, io.buffer_length());

        for i in 0..io.buffer_length() {
            println!("{}: {}", i, io.buffer_get(i));
        }

        if io.buffer_length() == 12 {
            assert!(io
                .buffer_get(3)
                .to_string()
                .eq(&"ok_message_for_try_login".to_string()));
            assert!(io.buffer_get(7).to_string().eq(&"org2".to_string()));
            assert!(io.buffer_get(8).to_string().eq(&" Slug: ".to_string()));
            assert!(io.buffer_get(9).to_string().eq(&"slug_org2".to_string()));
            assert!(io
                .buffer_get(10)
                .to_string()
                .eq(&"\nChoose organization by writing its slug: ".to_string()));
            assert!(io
                .buffer_get(11)
                .to_string()
                .eq(&"Could not set organization".to_string()));
        }
    }
}
