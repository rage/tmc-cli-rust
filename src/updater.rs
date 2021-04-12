use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Url};
use std::env;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use std::cmp::min;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::commands::command_util::get_path;
use tmc_langs::{ConfigValue, TmcConfig};

pub const GITHUB_URL: &str = "https://api.github.com/repos/rage/tmc-cli-rust/tags";
pub const PLUGIN: &str = "vscode_plugin";
pub const DELAY: u128 = 5 * 60 * 1000;

pub fn check_for_update() {
    if is_it_time_yet() {
        let new_ver = get_latest_version();
        if compare_versions(new_ver.clone()) {
            stash_old_executable();
            update(new_ver).unwrap();
            println!("Update completed succesfully!")
        }
        generate_time_stamp();
    }
}

fn is_it_time_yet() -> bool {
    let config = TmcConfig::load(PLUGIN, get_path().as_path()).unwrap();

    let last_check = match config.get("update-last-checked") {
        ConfigValue::Value(Some(s)) => toml::Value::as_str(&s).unwrap().to_string(),
        _ => {
            return true;
        }
    };

    let last_check = match last_check.parse::<u128>() {
        Ok(time) => time,
        _ => return true,
    };
    let now = SystemTime::now();
    let now = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    if now - last_check as u128 > DELAY {
        return true;
    }
    false
}

fn generate_time_stamp() {
    let mut config = TmcConfig::load(PLUGIN, get_path().as_path()).unwrap();
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    if let Err(_err) = config.insert(
        "update-last-checked".to_string(),
        toml::Value::String(since_the_epoch.to_string()),
    ) {
        println!("timestamp could not be changed");
    }
    if let Err(_err) = config.save(get_path().as_path()) {
        println!("Problem saving timestamp");
    }
}

fn get_latest_version() -> String {
    println!("Checking for updates...");
    let url = GITHUB_URL;
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        "tmc-cli-rust".parse().expect("github invalid user-agent"),
    );
    let resp = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .unwrap();
    if !resp.status().is_success() {
        panic!(
            "Version lookup failed with status: {:?} - for: {:?}",
            resp.status(),
            &url
        );
    }
    let tags = resp.json::<serde_json::Value>().unwrap();
    let tags = tags.as_array().unwrap();

    let latest = tags[0]["name"].to_string();

    latest[1..latest.len() - 1].to_string()
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

fn update(version: String) -> Result<(), Box<dyn std::error::Error>> {
    use io::BufRead;
    let resp = reqwest::blocking::Client::new()
        .get(generate_download_url(version))
        .send()?;
    let size = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .map(|val| {
            val.to_str()
                .map(|s| s.parse::<u64>().unwrap_or(0))
                .unwrap_or(0)
        })
        .unwrap_or(0);
    if !resp.status().is_success() {
        panic!("Download request failed with status: {:?}", resp.status());
    }
    let filepath = env::current_exe().unwrap();
    let mut dest = fs::File::create(&filepath).unwrap();
    let mut src = io::BufReader::new(resp);
    let mut downloaded = 0;
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));
    while downloaded < size {
        let n = {
            let buf = src.fill_buf()?;
            dest.write_all(&buf)?;
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

fn stash_old_executable() {
    let filepath = env::current_exe().unwrap();

    let mut tmp_filepath = env::current_exe().unwrap();
    tmp_filepath.pop();
    let tmp_filepath = Path::new(&tmp_filepath).join("tmp");
    fs::create_dir_all(&tmp_filepath).unwrap();
    let tmp_filepath = tmp_filepath.join("tmc.exe");

    if tmp_filepath.exists() {
        //fs::remove_file(&tmp_filepath).unwrap();
    }

    fs::rename(&filepath, &tmp_filepath).unwrap();
}

fn generate_download_url(version: String) -> Url {
    let arch = env::consts::ARCH;
    let mut target = String::new();
    match arch {
        "x86_64" => target.push_str("x86_64-pc-windows-msvc-"),
        "i686" => target.push_str("i686-pc-windows-msvc-"),
        _ => println!("Wow! {}", arch),
    }
    let mut download_url = String::from("https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-");
    download_url.push_str(&target);
    download_url.push_str(&version);
    download_url.push_str(".exe");
    Url::parse(&download_url).unwrap()
}

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
#[cfg(windows)]
fn generate_download_url_test() {
    let oldest = String::from("v0.0.1");
    let arch = env::consts::ARCH;
    match arch {
        "x86_64" => assert_eq!(
            generate_download_url(oldest).to_string(),
            "https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-x86_64-pc-windows-msvc-v0.0.1.exe"
        ),
        "i686" => assert_eq!(
            generate_download_url(oldest).to_string(),
            "https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-i686-pc-windows-msvc-v0.0.1.exe"
        ),
        _ => println!("Wow! {}", arch),
    }
}
