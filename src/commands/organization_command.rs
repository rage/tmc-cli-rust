use super::command_util;
use super::command_util::Client;
use crate::interactive::{self, interactive_list};
use crate::io_module::{Io, PrintColor};

// Asks for organization from user and saves it into file
pub fn set_organization_old(io: &mut dyn Io, client: &mut dyn Client) -> Result<String, String> {
    // List all organizations
    let mut orgs = client.get_organizations().unwrap();
    orgs.sort_by(|a, b| b.pinned.cmp(&a.pinned));
    let mut last_pinned = true;

    io.println("Available Organizations:", PrintColor::Normal);
    io.println("", PrintColor::Normal);

    for org in &orgs {
        if org.pinned != last_pinned {
            io.println("----------", PrintColor::Normal);
        }
        io.print(&org.name, PrintColor::Normal);
        io.print(" Slug: ", PrintColor::Normal);
        io.println(&org.slug, PrintColor::Normal);
        last_pinned = org.pinned;
    }

    io.print(
        "\nChoose organization by writing its slug: ",
        PrintColor::Normal,
    );
    let mut slug = io.read_line();
    slug = slug.trim().to_string();

    if let Some(org) = orgs.into_iter().find(|org| org.slug == slug) {
        command_util::set_organization(&slug).unwrap();
        return Ok(org.name);
    }

    Err(format!("No such organization for the given slug: {}", slug))
}

pub fn set_organization(io: &mut dyn Io, client: &mut dyn Client) -> Result<String, String> {
    io.println("Fetching organizations...", PrintColor::Normal);
    let mut orgs = client.get_organizations().unwrap();
    let mut pinned = orgs
        .iter()
        .filter(|org| org.pinned)
        .map(|org| org.name.clone())
        .collect::<Vec<_>>();

    orgs.sort_by(|a, b| b.pinned.cmp(&a.pinned));

    let others = String::from("View all organizations");
    pinned.push(others.clone());

    let prompt = String::from("Select your organization: ");
    let mut org_name = interactive::interactive_list(&prompt, pinned);

    org_name = match org_name {
        None => return Err("No organization chosen".to_string()),
        Some(result) if result.eq(&others) => {
            let all = orgs.iter().map(|org| org.name.clone()).collect();
            interactive_list(&prompt, all)
        }
        option => option,
    };

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
        io.println(&error, PrintColor::Failed);
        return;
    };

    let res = if interactive_mode {
        set_organization(io, client)
    } else {
        set_organization_old(io, client)
    };

    match res {
        Ok(org) => io.println(
            &format!("Selected {} as organization.", org),
            PrintColor::Success,
        ),
        Err(msg) => io.println(&msg, PrintColor::Failed),
    }
}
