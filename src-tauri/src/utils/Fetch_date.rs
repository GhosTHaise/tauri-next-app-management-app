use std::{fmt::format, thread, time::Duration};

use chrono::{DateTime,Local, Datelike, Timelike};
use tauri::{ Window};


#[tauri::command]
pub fn get_date() -> String{ 
    let current_time = Local::now();
    let weekDay_name = current_time.format("%A").to_string();
    let month_name = current_time.format("%B").to_string();
    let day = current_time.day();
    format!(" - {} {} {}",
    weekDay_name,
    month_name,
    day)
}

#[tauri::command]
pub fn get_instant_hour(window : Window){
    thread::spawn(move || {
        loop {
            let current_time = Local::now();
            window.emit("instant_hour", format!("{:02}:{:02}",
            current_time.hour(),
            current_time.minute()));
     
            thread::sleep(Duration::from_secs(60));
        }
    });
}   