use std::process::Command;

pub fn start_tor_proxy_container() -> Result<String, Box<dyn std::error::Error>> {
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