#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}
mod utils;
mod api;
fn main() {
  let mut app = tauri::Builder::default()
    .setup(|app|{
      let window = app.get_window("main").unwrap();
      window.emit("instant_hour","09-09");
      utils::Fetch_date::get_instant_hour(window);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![utils::Fetch_date::get_date])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
