use super::command_util::Client;
use super::organization_command::set_organization;
use crate::io_module::Io;

pub fn login(io: &mut dyn Io, client: &mut dyn Client) {
    if let Ok(()) = client.load_login() {
        io.println("You are already logged in.".to_string());
        return;
    };

    io.print("Email / username: ".to_string());
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!".to_string());
        return;
    }

    io.print("Password: ".to_string());
    let mut password = io.read_password();
    password = password.trim().to_string();

    io.println("".to_string());

    match client.try_login(username, password) {
        Ok(message) => {
            io.println(message);
            if let Err(_err) = set_organization(io, client) {
                io.println("Could not set organization".to_string());
            }
        }
        Err(message) => io.println(message),
    }
}

#[cfg(test)]
pub struct IoTest<'a> {
    list: &'a mut Vec<String>,
}

#[cfg(test)]
impl IoTest<'_> {
    // pub fn buffer_length(&mut self) -> usize {
    //     self.list.len()
    // }

    // pub fn buffer_get(&mut self, index: usize) -> String {
    //     self.list[index].to_string()
    // }
}

#[cfg(test)]
impl Io for IoTest<'_> {
    fn read_line(&mut self) -> String {
        "".to_string()
    }

    fn print(&mut self, output: String) {
        print!("{}", output);
        self.list.push(output);
    }

    fn println(&mut self, output: String) {
        println!("{}", output);
        self.list.push(output);
    }

    fn read_password(&mut self) -> String {
        "".to_string()
    }
}

// #[cfg(test)]
// pub struct ClientTest {
// }

// #[cfg(test)]
// impl ClientTest {
// }

// #[cfg(test)]
// impl Client for ClientTest {
//     fn load_login(&mut self) -> Result<(), String>;
//     fn try_login(&mut self, username: String, password: String) -> Result<String, String>;
//     fn list_courses(&mut self) -> Result<Vec<Course>, String>;
//     fn get_organizations(&mut self) -> Result<Vec<Organization>, String>;
//     fn logout(&mut self);
//     fn get_course_exercises(&mut self, course_id: usize) -> Result<Vec<CourseExercise>, String>;
//     fn download_or_update_exercises(&mut self, download_params: Vec<(usize, PathBuf)>) -> Result<String, String>;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_username_test() {
        let mut v: Vec<String> = Vec::new();
        let mut io = IoTest { list: &mut v };

        // let mut client = ClientTest { };

        assert!(io.read_line().eq(""));

        // login(&mut io, &mut client);

        // assert_eq!(2, io.buffer_length());

        // if io.buffer_length() == 2 {
        //     assert!(io
        //         .buffer_get(1)
        //         .to_string()
        //         .eq(&"Username cannot be empty!".to_string()));
        // }
    }
}
