// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

#[derive(Deserialize, Serialize)]
struct ProjectData {
    project: serde_json::Value,
}

fn get_project_file_path() -> Result<PathBuf, String> {
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(current_dir.join("test_project.json"))
}

#[tauri::command]
fn save_project_data(data: ProjectData) -> Result<String, String> {
    let json_content = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    let project_file_path = get_project_file_path()?;
    
    fs::write(&project_file_path, &json_content).map_err(|e| e.to_string())?;
    
    Ok(format!("Saved to: {}", project_file_path.display()))
}

#[tauri::command] 
fn load_project_data() -> Result<ProjectData, String> {
    let project_file_path = get_project_file_path()?;
    
    let content = fs::read_to_string(&project_file_path).map_err(|e| e.to_string())?;
    let data: ProjectData = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_available_ports_cmd,
            save_project_data,
            load_project_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
