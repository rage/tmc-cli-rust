use crate::io_module::IO;

pub fn login(io: &mut IO) {
    io.print(&format!("{}", "username: "));
    let mut username = io.read_line();
    username = username.trim().to_string();

    io.print(&"password: ".to_string());
    let mut password = io.read_line();
    password = password.trim().to_string();

    io.println(&"You entered:".to_string());
    io.println(&format!("\tusername: {}", username));
    io.println(&format!("\tpassword: {}", password));
}
