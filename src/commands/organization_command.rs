use super::command_util;
use super::command_util::Client;
use crate::io_module::Io;

// Asks for organization from user and saves it into file
pub fn set_organization(io: &mut dyn Io, client: &mut dyn Client) -> Result<String, String> {
    //List all organizations
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

#[cfg(target_family = "unix")]
pub fn set_organization_interactive(client: &mut dyn Client) -> Result<String, String> {
    use skim::prelude::*;
    use std::io::Cursor;

    let orgs = client.get_organizations().unwrap();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let input = orgs
        .iter()
        .map(|org| format!("{}", org.name))
        .collect::<Vec<_>>();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    if let Some(name) = selected_items.iter().next() {
        let name = name.output();
        if let Some(org) = orgs.into_iter().find(|org| org.name == name) {
            command_util::set_organization(&org.slug).unwrap();
            return Ok(org.name);
        }
    }

    Err(format!("No such organization"))
}

// Check if logged in, then ask for organization
pub fn organization(io: &mut dyn Io, client: &mut dyn Client, interactive: bool) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    if !interactive || cfg!(not(unix)) {
        match set_organization(io, client) {
            Ok(org) => io.println(&format!("Selected {} as organization.", org)),
            Err(msg) => io.println(&msg),
        }
    } else {
        match set_organization_interactive(client) {
            Ok(org) => io.println(&format!("Selected {} as organization.", org)),
            Err(msg) => io.println(&msg),
        }
    }
}
