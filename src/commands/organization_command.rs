use super::command_util::*;
use crate::io_module::IO;
use anyhow::Result;
use std::path::PathBuf;
use tmc_langs_framework::file_util;

pub fn get_organization_path(client_name: &str) -> Result<PathBuf> {
    crate::config::get_tmc_dir(client_name).map(|dir| dir.join("organization.json"))
}

// Returns none if no valid organization slug can be read from file,
// otherwise organization slug as string
pub fn check_organization(client_name: String) -> Option<String> {
    // TBD Read organization from file
    // for now just return mooc
    Some("mooc".to_string())
}

// Asks for organization from user and saves it into file
pub fn set_organization(io: &mut IO) -> Result<()> {
    // List all organizations
    for org in get_client().get_organizations().unwrap() {
        io.print(org.name);
        io.print(" Slug: ");
        io.println(org.slug);
    }

    io.print("Choose organization by writing its slug: ");
    let mut slug = io.read_line();
    slug = slug.trim().to_string();

    // TBD: Save organization into file here
    io.println("TBD: SAVING ORGANIZATION INTO FILE");
    Ok(())
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut IO) {
    if !is_logged_in() {
        io.println("No login found. You need to be logged in to set organization.");
        return;
    }

    set_organization(io);
}
