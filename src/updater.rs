use crate::commands::util::get_path;
use anyhow::Context;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{
    header::{self, HeaderValue},
    Url,
};
use std::{
    cmp::min,
    env, fs, io,
    io::Write,
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use tmc_langs::{ConfigValue, TmcConfig};

pub const GITHUB_URL: &str = "https://api.github.com/repos/rage/tmc-cli-rust/tags";
pub const PLUGIN: &str = "tmc_cli_rust";
pub const DELAY_MILLIS_24H: u128 = 1440 * 60 * 1000;

/// Autoupdater for Windows platform.
/// Checks every 24 hours if there are new versions available and
/// generates a new timestamp. If a new version is found, the function
/// stashes the old executable and downloads a new one.
/// Will run in privileged stage if needed on Windows!

pub fn check_for_update(force: bool) -> anyhow::Result<()> {
    if force || is_it_time_yet()? {
        generate_time_stamp()?;
        checktemp()?;
        let new_ver = get_latest_version()?;
        println!("Checking for updates...");
        if compare_versions(new_ver) {
            process_update()?;
        }
    }
    Ok(())
}

fn checktemp() -> anyhow::Result<()> {
    let mut tmp_filepath = env::current_exe()?;
    tmp_filepath.pop();
    let tmp_filepath = Path::new(&tmp_filepath).join("tmp");
    let tmp_filepath = tmp_filepath.join("tmc.exe");
    if tmp_filepath.exists() {
        if let Err(e) = cleartemp() {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    println!("Permission Denied! Restarting with administrator privileges...");
                    elevate("cleartemp".to_string())?;
                }
                _ => {
                    println!("{:#?}", e);
                }
            }
        }
    }
    Ok(())
}

pub fn cleartemp() -> Result<(), std::io::Error> {
    println!("Cleaning temp...");
    let mut tmp_filepath = env::current_exe()?;
    tmp_filepath.pop();
    let tmp_filepath = Path::new(&tmp_filepath).join("tmp");
    let tmp_filepath = tmp_filepath.join("tmc.exe");
    fs::remove_file(&tmp_filepath)?;
    println!("Temp cleared!");
    Ok(())
}

pub fn process_update() -> anyhow::Result<()> {
    let new_ver = get_latest_version()?;
    match stash_old_executable() {
        Err(e) => match e.downcast_ref::<std::io::Error>().map(|e| e.kind()) {
            Some(std::io::ErrorKind::PermissionDenied) => {
                println!("Permission Denied! Restarting with administrator privileges...");
                elevate("fetchupdate".to_string())?;
                return Ok(());
            }
            _ => {
                println!("{:#?}", e);
            }
        },
        _ => update(new_ver)?,
    }
    println!("Update completed succesfully!");
    Ok(())
}

fn elevate(command: String) -> anyhow::Result<()> {
    Command::new("powershell")
        .args(&[
            "-Command",
            "Start-Process",
            "tmc.exe",
            &command,
            "-Verb",
            "RunAs",
        ])
        .spawn()
        .context("launch failure")?;
    Ok(())
}

fn is_it_time_yet() -> anyhow::Result<bool> {
    let config = TmcConfig::load(PLUGIN, get_path()?.as_path())?;

    let value = config.get("update-last-checked");
    let last_check = match &value {
        ConfigValue::Value(Some(s)) => s.as_str().context("invalid value")?,
        _ => {
            return Ok(true);
        }
    };

    let last_check = match last_check.parse::<u128>() {
        Ok(time) => time,
        _ => return Ok(true),
    };
    let now = SystemTime::now();
    let now = now
        .duration_since(UNIX_EPOCH)
        .context("Time went backwards")?
        .as_millis();
    let update = now - last_check as u128 > DELAY_MILLIS_24H;
    Ok(update)
}

fn generate_time_stamp() -> anyhow::Result<()> {
    let mut config = TmcConfig::load(PLUGIN, get_path()?.as_path())?;
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .context("Time went backwards")?
        .as_millis();

    if let Err(_err) = config.insert(
        "update-last-checked".to_string(),
        toml::Value::String(since_the_epoch.to_string()),
    ) {
        println!("timestamp could not be changed");
    }
    if let Err(_err) = config.save(get_path()?.as_path()) {
        println!("Problem saving timestamp");
    }
    Ok(())
}

fn get_latest_version() -> anyhow::Result<String> {
    let url = GITHUB_URL;
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_static("tmc-cli-rust"));
    let resp = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers)
        .send()?;
    if !resp.status().is_success() {
        anyhow::bail!(
            "Version lookup failed with status: {:?} - for: {:?}",
            resp.status(),
            &url
        );
    }
    let tags = resp.json::<serde_json::Value>()?;
    let tags = tags.as_array().context("tags were not an array")?;

    let latest = tags[0]["name"].to_string();
    let latest = latest[1..latest.len() - 1].to_string();
    Ok(latest)
}

fn compare_versions(version: String) -> bool {
    let cur_ver = env!("CARGO_PKG_VERSION");
    let mut new_ver = String::new();
    for ch in version.chars() {
        if ch.is_alphabetic() {
            continue;
        } else {
            new_ver.push(ch);
        }
    }
    let cur_values: Vec<&str> = cur_ver.split('.').collect();
    let new_values: Vec<&str> = new_ver.split('.').collect();
    for (i, _x) in cur_values.iter().enumerate() {
        if cur_values[i] > new_values[i] {
            break;
        }
        if new_values[i] > cur_values[i] {
            println!("New version found! {}", version);
            return true;
        }
    }
    println!("Application is up-to-date!");
    false
}

fn update(version: String) -> anyhow::Result<()> {
    use io::BufRead;
    let resp = reqwest::blocking::Client::new()
        .get(generate_download_url(version)?)
        .send()?;
    let size = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok().and_then(|s| s.parse::<u64>().ok()))
        .unwrap_or(0);
    if !resp.status().is_success() {
        anyhow::bail!("Download request failed with status: {:?}", resp.status());
    }
    let filepath = env::current_exe()?;
    let mut dest = fs::File::create(&filepath)?;
    let mut src = io::BufReader::new(resp);
    let mut downloaded = 0;
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));
    while downloaded < size {
        let n = {
            let buf = src.fill_buf()?;
            dest.write_all(buf)?;
            buf.len()
        };
        if n == 0 {
            break;
        }
        src.consume(n);
        downloaded = min(downloaded + n as u64, size);
        pb.set_position(downloaded);
    }
    pb.finish_with_message("downloaded");
    println!("Download complete");
    Ok(())
}

fn stash_old_executable() -> anyhow::Result<()> {
    let filepath = env::current_exe()?;

    let mut tmp_filepath = filepath.clone();
    tmp_filepath.pop();
    let tmp_dir = tmp_filepath.join("tmp");
    fs::create_dir_all(&tmp_dir)?;
    let tmp_tmc = tmp_dir.join("tmc.exe");
    fs::rename(&filepath, &tmp_tmc)?;
    Ok(())
}

fn generate_download_url(version: String) -> anyhow::Result<Url> {
    let arch = env::consts::ARCH;
    let target = match arch {
        "x86_64" => "x86_64-pc-windows-msvc",
        "i686" => "i686-pc-windows-msvc",
        unexpected => anyhow::bail!("Unexpected arch {unexpected}"),
    };
    let download_url =
        format!("https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-{target}-{version}.exe");
    let url = Url::parse(&download_url)?;
    Ok(url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_versions_test() {
        let oldest = String::from("v0.0.1");
        let the_future = String::from("v999.999.999");
        assert_eq!(compare_versions(oldest), false);
        assert_eq!(compare_versions(the_future), true);
        assert_eq!(
            compare_versions(env!("CARGO_PKG_VERSION").to_string()),
            false
        );
    }

    #[test]
    fn generate_download_url_test() {
        let oldest = String::from("v0.0.1");
        let arch = env::consts::ARCH;
        match arch {
            "x86_64" => assert_eq!(
            generate_download_url(oldest).unwrap().to_string(),
            "https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-x86_64-pc-windows-msvc-v0.0.1.exe"
        ),
            "i686" => assert_eq!(
            generate_download_url(oldest).unwrap().to_string(),
            "https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-i686-pc-windows-msvc-v0.0.1.exe"
        ),
            _ => println!("Wow! {}", arch),
        }
    }
}
