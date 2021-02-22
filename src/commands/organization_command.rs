use super::command_util;
use crate::io_module::Io;
use anyhow::Result;
use tmc_client::Organization;

// Asks for organization from user and saves it into file
pub fn set_organization(io: &mut dyn Io) -> Result<Organization> {
    // List all organizations
    let orgs = command_util::get_client().get_organizations().unwrap();
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
        return Ok(org);
    }

    anyhow::bail!("No such organization for the given slug: {}", slug);
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut dyn Io) {
    if !command_util::is_logged_in() {
        io.println("No login found. You need to be logged in to set organization.".to_string());
        return;
    }

    match set_organization(io) {
        Ok(org) => io.println(format!("Selected {} as organization.", org.name)),
        Err(msg) => io.println(msg.to_string()),
    }
}
