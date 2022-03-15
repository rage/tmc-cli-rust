use super::util::{get_projects_dir, Client};
use crate::io::{Io, PrintColor};
use std::path::{Path, PathBuf};
use std::process::Command;

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
pub fn update(io: &mut dyn Io, client: &mut dyn Client, currentdir: bool) {
    // Get a client that has credentials
    if let Err(error) = client.load_login() {
        io.println(&error, PrintColor::Failed);
        return;
    };
    let pathbuf = if currentdir {
        std::env::current_dir().unwrap()
    } else {
        get_projects_dir()
    };
    let tmp_path = &pathbuf;
    let tmp_path = tmp_path.to_str().unwrap();
    match call_update(&pathbuf, client) {
        Ok(msg) => io.println(&format!("\n{}", msg), PrintColor::Success),
        Err(msg) => {
            let os = std::env::consts::OS;
            if os == "windows" && msg.contains("Failed to create file") {
                io.println(
                    "Starting new cmd with administrator privileges...",
                    PrintColor::Normal,
                );
                let temp_file_path = get_projects_dir();
                let temp_file_path = temp_file_path.join("temp.txt");
                std::fs::write(temp_file_path, tmp_path).unwrap();
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
                    .expect("launch failure");
            } else {
                io.println(&msg, PrintColor::Failed)
            }
        }
    }
}
fn call_update(path: &Path, client: &mut dyn Client) -> Result<String, String> {
    let result = client.update_exercises(path);
    match result {
        Ok(_) => Ok(format!(
            "Exercises updated succesfully to {}\\",
            path.to_str().unwrap()
        )),
        Err(err) => Err(format!("Error: {}", err)),
    }
}

pub fn elevated_update(io: &mut dyn Io, client: &mut dyn Client) {
    use std::io::prelude::*;
    let temp_file_path = get_projects_dir();
    let temp_file_path = temp_file_path.join("temp.txt");
    let mut file = std::fs::File::open(temp_file_path.clone()).unwrap();
    let mut params = String::new();
    file.read_to_string(&mut params).unwrap();
    std::fs::remove_file(temp_file_path).unwrap();
    let path = PathBuf::from(params);
    io.println("", PrintColor::Normal);
    match call_update(&path, client) {
        Ok(msg) => io.println(&msg, PrintColor::Success),
        Err(msg) => io.println(&msg, PrintColor::Failed),
    }
    pause();
}
fn pause() {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any enter to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
