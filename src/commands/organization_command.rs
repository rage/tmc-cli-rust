use super::command_util;
use super::command_util::Client;
use crate::io_module::Io;

// Asks for organization from user and saves it into file
pub fn set_organization(io: &mut dyn Io, client: &mut dyn Client) -> Result<String, String> {
    // List all organizations
    let orgs = client.get_organizations().unwrap();
    for org in &orgs {
        io.print(&org.name);
        io.print(" Slug: ");
        io.println(&org.slug);
    }

    io.print("\nChoose organization by writing its slug: ");
    let mut slug = io.read_line();
    slug = slug.trim().to_string();

    if let Some(org) = orgs.into_iter().find(|org| org.slug == slug) {
        command_util::set_organization(&slug).unwrap();
        return Ok(org.name);
    }

    Err(format!("No such organization for the given slug: {}", slug))
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    match set_organization(io, client) {
        Ok(org) => io.println(&format!("Selected {} as organization.", org)),
        Err(msg) => io.println(&msg),
    }
}
