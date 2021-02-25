use super::command_util;
use crate::io_module::Io;
use tmc_client::Organization;
use super::command_util::Client;

// Asks for organization from user and saves it into file
pub fn set_organization(io: &mut dyn Io, client: &mut Client) -> Result<String, String> {
    // List all organizations
    let orgs = client.get_organizations().unwrap();
    for org in &orgs {
        io.print(org.name.to_string());
        io.print(" Slug: ".to_string());
        io.println(org.slug.to_string());
    }

    io.print("\nChoose organization by writing its slug: ".to_string());
    let mut slug = io.read_line();
    slug = slug.trim().to_string();

    if let Some(org) = orgs.into_iter().find(|org| org.slug == slug) {
        command_util::set_organization(&slug).unwrap();
        return Ok(org.name);
    }

    Err(format!("No such organization for the given slug: {}", slug).to_string())
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut dyn Io, client: &mut Client) {

    if let Err(error) = client.load_login() {
        io.println(error);
        return;
    };

    match set_organization(io, client) {
        Ok(org) => io.println(format!("Selected {} as organization.", org)),
        Err(msg) => io.println(msg.to_string()),
    }
}
