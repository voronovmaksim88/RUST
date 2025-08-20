// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;

mod scan_available_ports;

#[derive(Serialize)]
struct PortsResult {
    ports: Vec<u8>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn scan_available_ports_cmd() -> PortsResult {
    let mut buffer: [u8; 10] = [0; 10];
    let count = scan_available_ports::scan_available_ports(&mut buffer);
    PortsResult {
        ports: buffer[..count].to_vec(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, scan_available_ports_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
