use super::command_util::*;
use crate::io_module::Io;

pub fn logout(io: &mut dyn Io) {
    if !is_logged_in() {
        io.println("No login found".to_string());
        return;
    }

    let credentials = get_credentials().unwrap();

    credentials.remove().unwrap();
    io.println("Logged out successfully.".to_string());
}
