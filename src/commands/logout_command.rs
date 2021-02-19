use super::command_util::*;
use crate::io_module::Io;

pub fn logout(io: &mut Io) {
    if !is_logged_in() {
        io.println("No login found");
        return;
    }

    let credentials = get_credentials().unwrap();

    credentials.remove().unwrap();
    io.println("Logged out successfully.");
}
