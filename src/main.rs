mod docker;
mod config;
mod proxy;

use cron::Schedule;
use chrono::Local;

use std::thread;
use std::time::Duration;


use futures::executor::block_on;

#[tokio::main]
async fn main() {
    // docker::start_tor_proxy_container();
    let cron = config::load_config();
    let schedule: Schedule = cron.parse().unwrap(); // Run every minute
    loop {
        block_on(proxy::get_request_with_proxy()).unwrap();
        let next = schedule.upcoming(Local).next().unwrap();
        let sleep_duration = next - Local::now();
        let duration = Duration::from_secs(sleep_duration.num_seconds() as u64);
        thread::sleep(duration);
    }
}
