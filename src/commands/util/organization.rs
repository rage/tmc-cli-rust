use crate::{
    client::Client,
    interactive,
    io::{Io, PrintColor},
};
use tmc_langs::tmc::response::Organization;

pub fn select_organization(io: &mut Io, client: &mut Client) -> anyhow::Result<String> {
    let org = select_organization_inner(io, client)?;

    io.println(
        &format!("Selected {} as organization.", org.name),
        PrintColor::Success,
    )?;
    Ok(org.slug)
}

fn select_organization_inner(io: &mut Io, client: &mut Client) -> anyhow::Result<Organization> {
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
    let selection = interactive::interactive_list(&prompt, &pinned, None)?
        .ok_or_else(|| anyhow::anyhow!("Did not select any organization"))?;
    let org_name = if selection == others {
        let all = orgs.iter().map(|org| org.name.as_str()).collect::<Vec<_>>();
        interactive::interactive_list(&prompt, &all, None)?
            .ok_or_else(|| anyhow::anyhow!("Did not select any organization"))?
    } else {
        selection
    };

    if let Some(org) = orgs.into_iter().find(|org| org.name == org_name) {
        return Ok(org);
    }

    anyhow::bail!("Something strange happened");
}
