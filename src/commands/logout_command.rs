use super::command_util::*;
use crate::io_module::IO;

pub fn logout(io: &mut IO) {
    if !is_logged_in() {
        io.println("No login found");
        return;
    }

    let credentials = get_credentials().unwrap();

    credentials.remove().unwrap();
    io.println("Logged out successfully.");
}
