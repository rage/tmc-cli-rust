use super::command_util;
use super::command_util::Client;
use crate::interactive;
use crate::io_module::Io;

// Asks for organization from user and saves it into file
pub fn set_organization_old(io: &mut dyn Io, client: &mut dyn Client) -> Result<String, String> {
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

pub fn set_organization(client: &mut dyn Client) -> Result<String, String> {
    let orgs = client.get_organizations().unwrap();

    let org_name = interactive::interactive_list(
        "Select your organization:",
        orgs.iter().map(|org| org.name.clone()).collect(),
    );

    if org_name.is_none() {
        return Err("No organization chosen".to_string());
    }

    let org_name = org_name.unwrap();

    if let Some(org) = orgs.iter().find(|org| org.name == org_name) {
        command_util::set_organization(&org.slug).unwrap();
        return Ok(org.name.to_owned());
    }

    Err("Something strange happened".to_string())
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut dyn Io, client: &mut dyn Client, interactive_mode: bool) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };
    let res = if interactive_mode {
        set_organization(client)
    } else {
        set_organization_old(io, client)
    };
    match res {
        Ok(org) => io.println(&format!("Selected {} as organization.", org)),
        Err(msg) => io.println(&msg),
    }
}
