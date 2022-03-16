use super::util::{get_projects_dir, Client};
use crate::io::{Io, PrintColor};
use anyhow::Context;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
pub fn update(io: &mut dyn Io, client: &mut dyn Client, currentdir: bool) -> anyhow::Result<()> {
    // Get a client that has credentials
    client.load_login()?;
    let pathbuf = if currentdir {
        std::env::current_dir()?
    } else {
        get_projects_dir()?
    };
    let tmp_path = &pathbuf;
    let tmp_path = tmp_path.to_str().context("invalid path")?;
    match call_update(&pathbuf, client) {
        Ok(msg) => io.println(&format!("\n{}", msg), PrintColor::Success)?,
        Err(msg) => {
            let os = std::env::consts::OS;
            if os == "windows"
                && msg
                    .chain()
                    .any(|e| e.to_string().contains("Failed to create file"))
            {
                io.println(
                    "Starting new cmd with administrator privileges...",
                    PrintColor::Normal,
                )?;
                let temp_file_path = get_projects_dir()?;
                let temp_file_path = temp_file_path.join("temp.txt");
                std::fs::write(temp_file_path, tmp_path)?;
                Command::new("cmd")
                    .args(&[
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
                anyhow::bail!(msg);
            }
        }
    }
    Ok(())
}

fn call_update(path: &Path, client: &mut dyn Client) -> anyhow::Result<String> {
    client.update_exercises(path)?;
    Ok(format!(
        "Exercises updated succesfully to {}",
        path.to_str().context("invalid path")?
    ))
}

pub fn elevated_update(io: &mut dyn Io, client: &mut dyn Client) -> anyhow::Result<()> {
    use std::io::prelude::*;
    let temp_file_path = get_projects_dir()?;
    let temp_file_path = temp_file_path.join("temp.txt");
    let mut file = std::fs::File::open(temp_file_path.clone())?;
    let mut params = String::new();
    file.read_to_string(&mut params)?;
    std::fs::remove_file(temp_file_path)?;
    let path = PathBuf::from(params);
    io.println("", PrintColor::Normal)?;
    let msg = call_update(&path, client)?;
    io.println(&msg, PrintColor::Success)?;
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
