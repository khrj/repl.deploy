use {
    chrono::{offset::Utc, Datelike, Timelike},
    colored::*,
    std::process,
};

pub fn success(s: &str) {
    println!("{}", pad_str_with_time(s).green());
}

pub fn info(s: &str) {
    println!("{}", pad_str_with_time(s).blue());
}

pub fn warn(s: &str) {
    println!("{}", pad_str_with_time(s).yellow());
}

pub fn error(s: &str) {
    println!("{}", pad_str_with_time(s).red());
}

pub fn fatal_error(s: &str) {
    error(s);
    process::exit(0);
}

fn pad_str_with_time(to_pad: &str) -> String {
    let current_time = Utc::now();

    format!(
        "{:04}/{:02}/{:02} {:02}:{:02}:{:02} {}",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
        to_pad
    )
}