#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}
mod utils;
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      greet,
      utils::Fetch_date::get_date
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
