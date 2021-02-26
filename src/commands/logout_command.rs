use super::command_util::Client;
use crate::io_module::Io;

pub fn logout(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    client.logout();
    io.println("Logged out successfully.");
}
