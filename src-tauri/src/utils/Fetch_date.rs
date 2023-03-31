use std::fmt::format;

use chrono::{DateTime,Local, Datelike, Timelike};

#[tauri::command]
pub fn get_date() -> String{ 
    let current_time = Local::now();
    let weekDay_name = current_time.weekday().to_string();
    let month_name = current_time.month();
    let day = current_time.day();
    format!("{:02}:{:02} - {} {} {}",
    current_time.hour(),
    current_time.minute(),
    weekDay_name,
    month_name,
    day)
}