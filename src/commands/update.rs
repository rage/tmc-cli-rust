use super::{mooc, util, Platform};
use crate::{
    client::Client,
    config::TmcCliConfig,
    io::{Io, PrintColor},
};
use anyhow::Context;
use std::{path::PathBuf, process::Command};

pub fn update(
    io: &mut Io,
    client: &mut Client,
    config: &mut TmcCliConfig,
    current_dir: bool,
) -> anyhow::Result<()> {
    util::ensure_logged_in(client, io, config)?;
    match util::select_courses_or_tmc()? {
        Platform::Mooc => {
            mooc::update_exercises::run(io, client, config)?;
        }
        Platform::Tmc => {
            tmc_update(io, client, current_dir, config)?;
        }
    };
    Ok(())
}

/// Updates exercises from project dir or current directory.
/// Update is ran only if local exercise checksums differ from
/// the server exercises. Download_or_update also updates old
/// exercises, however, this function doesn't download them.
/// Will run in privileged stage if needed on Windows!
///
/// # Errors
/// May return random errors :)
///
/// # Usage:
/// tmc update //runs update command in project dir
/// tmc update -d //runs update command in current dir
///
fn tmc_update(
    io: &mut Io,
    client: &mut Client,
    current_dir: bool,
    config: &mut TmcCliConfig,
) -> anyhow::Result<()> {
    // Get a client that has credentials
    client.load_login(config)?;
    let path = if current_dir {
        std::env::current_dir()?
    } else {
        config.get_projects_dir().to_path_buf()
    };
    let tmp_path = path.to_str().context("invalid path")?;
    match client.update_exercises(&path) {
        Ok(_) => {
            io.println(
                &format!(
                    "Exercises updated succesfully to {}",
                    path.to_str().context("invalid path")?
                ),
                PrintColor::Success,
            )?;
        }
        Err(err) => {
            let os = std::env::consts::OS;
            let err = anyhow::format_err!(err);
            if os == "windows"
                && err
                    .chain()
                    .any(|e| e.to_string().contains("Failed to create file"))
            {
                io.println(
                    "Starting new cmd with administrator privileges...",
                    PrintColor::Normal,
                )?;
                let temp_file_path = config.get_projects_dir();
                let temp_file_path = temp_file_path.join("temp.txt");
                std::fs::write(temp_file_path, tmp_path)?;
                Command::new("cmd")
                    .args([
                        "/C",
                        "powershell",
                        "-Command",
                        "Start-Process",
                        "tmc.exe",
                        "elevatedupdate",
                        "-Verb",
                        "RunAs",
                    ])
                    .spawn()
                    .context("launch failure")?;
            } else {
                anyhow::bail!(err);
            }
        }
    }
    Ok(())
}

pub fn elevated_update(
    io: &mut Io,
    client: &mut Client,
    config: &mut TmcCliConfig,
) -> anyhow::Result<()> {
    util::ensure_logged_in(client, io, config)?;
    match util::select_courses_or_tmc()? {
        Platform::Mooc => {
            todo!()
        }
        Platform::Tmc => {
            elevated_tmc_update(io, client, config)?;
        }
    };
    Ok(())
}

fn elevated_tmc_update(
    io: &mut Io,
    client: &mut Client,
    config: &TmcCliConfig,
) -> anyhow::Result<()> {
    use std::io::prelude::*;
    let temp_file_path = config.get_projects_dir();
    let temp_file_path = temp_file_path.join("temp.txt");
    let mut file = std::fs::File::open(temp_file_path.clone())?;
    let mut params = String::new();
    file.read_to_string(&mut params)?;
    std::fs::remove_file(temp_file_path)?;
    let path = PathBuf::from(params);
    io.println("", PrintColor::Normal)?;
    client.update_exercises(&path)?;
    io.println(
        &format!(
            "Exercises updated succesfully to {}",
            path.to_str().context("invalid path")?
        ),
        PrintColor::Success,
    )?;
    pause()?;
    Ok(())
}

fn pause() -> anyhow::Result<()> {
    use std::{io, io::prelude::*};
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any enter to continue...")?;
    stdout.flush()?;
    let _ = stdin.read(&mut [0u8])?;
    Ok(())
}
