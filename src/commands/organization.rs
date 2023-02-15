use super::{util, util::Client};
use crate::{
    interactive::{self, interactive_list},
    io::{Io, PrintColor},
};

// Asks for organization from user and saves it into file
pub fn set_organization_old(io: &mut dyn Io, client: &mut dyn Client) -> anyhow::Result<String> {
    // List all organizations
    let mut orgs = client.get_organizations()?;
    orgs.sort_by(|a, b| b.pinned.cmp(&a.pinned).then(b.name.cmp(&a.name)));
    let mut last_pinned = true;

    io.println("Available Organizations:", PrintColor::Normal)?;
    io.println("", PrintColor::Normal)?;

    for org in &orgs {
        if org.pinned != last_pinned {
            io.println("----------", PrintColor::Normal)?;
        }
        io.print(&org.name, PrintColor::Normal)?;
        io.print(" Slug: ", PrintColor::Normal)?;
        io.println(&org.slug, PrintColor::Normal)?;
        last_pinned = org.pinned;
    }

    io.print(
        "\nChoose organization by writing its slug: ",
        PrintColor::Normal,
    )?;
    let slug = io.read_line()?.trim().to_string();

    if let Some(org) = orgs.into_iter().find(|org| org.slug == slug) {
        util::set_organization(slug)?;
        return Ok(org.name);
    }

    anyhow::bail!("No such organization for the given slug: {}", slug);
}

pub fn set_organization(io: &mut dyn Io, client: &mut dyn Client) -> anyhow::Result<String> {
    io.println("Fetching organizations...", PrintColor::Normal)?;
    let mut orgs = client.get_organizations()?;
    orgs.sort_by(|a, b| b.pinned.cmp(&a.pinned).then(a.name.cmp(&b.name)));
    let mut pinned = orgs
        .iter()
        .filter(|org| org.pinned)
        .map(|org| org.name.as_str())
        .collect::<Vec<_>>();

    let others = String::from("View all organizations");
    pinned.push(others.as_str());

    let prompt = String::from("Select your organization: ");
    let selection = interactive::interactive_list(&prompt, &pinned)?
        .ok_or_else(|| anyhow::anyhow!("Didn't select any organization"))?;
    let org_name = if selection == others {
        let all = orgs.iter().map(|org| org.name.as_str()).collect::<Vec<_>>();
        interactive_list(&prompt, &all)?
            .ok_or_else(|| anyhow::anyhow!("Didn't select any organization"))?
    } else {
        selection
    };

    if let Some(org) = orgs.into_iter().find(|org| org.name == org_name) {
        util::set_organization(org.slug)?;
        return Ok(org.name);
    }

    anyhow::bail!("Something strange happened");
}

// Check if logged in, then ask for organization
pub fn organization(
    io: &mut dyn Io,
    client: &mut dyn Client,
    interactive_mode: bool,
) -> anyhow::Result<()> {
    client.load_login()?;

    let org = if interactive_mode {
        set_organization(io, client)?
    } else {
        set_organization_old(io, client)?
    };

    io.println(
        &format!("Selected {org} as organization."),
        PrintColor::Success,
    )?;
    Ok(())
}
