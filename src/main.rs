use cron::Schedule;
use chrono::{Local};
use reqwest::header::{HeaderName, HeaderValue};
use std::fs;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use futures::executor::block_on;
use reqwest::header::HeaderMap;

#[tokio::main]
async fn main() {
    let start_container_result = start_tor_proxy_container();
    if (start_container_result.is_err()) {
        stop_tor_proxy_container();
    }
    let schedule: Schedule = "0 0 0 */3 * *".parse().unwrap(); // Run every minute
    loop {
        block_on(get_request_with_proxy()).unwrap();
        let next = schedule.upcoming(Local).next().unwrap();
        let sleep_duration = next - Local::now();
        let duration = Duration::from_secs(sleep_duration.num_seconds() as u64);
        println!("{}", duration.as_secs());
        thread::sleep(duration);
        
    }
}

async fn get_request_with_proxy() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("socks5://localhost:9050")?)
        .build()?;
    let request_builder = client.get("https://api.spotify.com/v1/me");
    let headers = get_headers_from_file("headers.txt")?;
    let response = request_builder.headers(headers).send().await?;

    println!("{}", response.text().await?);
    Ok(())
}

fn get_headers_from_file(filename: &str) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    for line in lines {
        let header_parts: Vec<&str> = line.split(':').collect();
        let mut header_name = String::from(header_parts[0].trim());
        let mut header_value = String::from(header_parts[1].trim());
        if header_name.len() == 0 {
            header_name = header_parts[1].trim().to_string();
            header_value = header_parts[2].trim().to_string();
        }
        let value = HeaderValue::from_str(header_value.as_str())?;
        let key = HeaderName::from_str(header_name.as_str())?;
        header_map.append(key, value);
    }
    Ok(header_map)
}

fn start_tor_proxy_container() -> Result<String, Box<dyn std::error::Error>> {
    // Start a Docker container
    // docker ps -aqf "name=containername"
    let output = Command::new("docker")
        .arg("run")
        .arg("-p")
        .arg("9050:9050")
        .arg("-e")
        .arg("LOCATION=IN")
        .arg("-d")
        .arg("--name")
        .arg("torproxy")
        .arg("dperson/torproxy")
        .output()?;

    // Get the container ID from the output
    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("Started container with ID: {}", container_id);

    Ok(container_id)
}

fn stop_tor_proxy_container() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-aqf")
        .arg("name=torproxy")
        .output()?;

    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let _output = Command::new("docker")
        .arg("stop")
        .arg(&container_id)
        .output()?;
    println!("Stopped container with ID: {}", container_id);

    Ok(())
}
