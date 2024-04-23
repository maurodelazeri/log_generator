use chrono::prelude::*;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let path = "test_logs.log"; // Path to the log file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .expect("Cannot open file");

    loop {
        let burst_size = rand::thread_rng().gen_range(1..=1000);
        for _ in 0..burst_size {
            let log_entry = generate_log_entry();
            writeln!(file, "{}", log_entry).expect("Failed to write to file");
            file.flush().expect("Failed to flush");
        }

        let sleep_time = rand::thread_rng().gen_range(1..=10);
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

fn generate_log_entry() -> String {
    let level = match rand::thread_rng().gen_range(0..=2) {
        0 => "ERROR",
        1 => "INFO",
        _ => "DEBUG",
    };
    let ip = format!("192.168.{}.{}", rand::thread_rng().gen_range(1..=254), rand::thread_rng().gen_range(1..=254));
    let error_message = generate_error_message(level);
    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    format!("[{}] {} - IP:{} {}", timestamp, level, ip, error_message)
}

fn generate_error_message(level: &str) -> String {
    if level == "ERROR" {
        let errors = [
            "Database connection failed",
            "Null pointer exception",
            "File not found",
            "Access denied",
            "Out of memory",
            "Network timeout occurred",
            "Illegal argument provided",
            "User authentication failed",
        ];
        let error = errors[rand::thread_rng().gen_range(0..errors.len())];
        format!("Error 500 - {}", error)
    } else {
        "".to_string()
    }
}
