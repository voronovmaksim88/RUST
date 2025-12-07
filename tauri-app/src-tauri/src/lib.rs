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
    println!("Current directory: {:?}", current_dir);

    // Check if we're running from src-tauri directory (dev mode)
    let parent_dir = if current_dir.file_name().and_then(|n| n.to_str()) == Some("src-tauri") {
        current_dir.parent().unwrap_or(&current_dir).to_path_buf()
    } else {
        current_dir.clone()
    };

    println!("Working directory adjusted to: {:?}", parent_dir);

    // In dev mode, use public/test_project.json
    // In production, use test_project.json in the same directory as the exe
    let public_path = parent_dir.join("public").join("test_project.json");
    let prod_path = parent_dir.join("test_project.json");

    println!(
        "Checking public path: {:?} (exists: {})",
        public_path,
        public_path.exists()
    );
    println!(
        "Checking prod path: {:?} (exists: {})",
        prod_path,
        prod_path.exists()
    );

    if public_path.exists() {
        println!("Using public path: {:?}", public_path);
        Ok(public_path)
    } else {
        println!("Using prod path: {:?}", prod_path);
        Ok(prod_path)
    }
}

#[tauri::command]
fn save_project_data(data: ProjectData) -> Result<String, String> {
    println!("save_project_data called");
    let json_content = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    let project_file_path = get_project_file_path()?;

    println!("About to write to: {:?}", project_file_path);
    fs::write(&project_file_path, &json_content).map_err(|e| {
        println!("Error writing file: {}", e);
        e.to_string()
    })?;
    println!("Successfully wrote to: {:?}", project_file_path);

    Ok(format!("Saved to: {}", project_file_path.display()))
}

#[tauri::command]
fn load_project_data() -> Result<ProjectData, String> {
    println!("load_project_data called");
    let project_file_path = get_project_file_path()?;

    println!("Attempting to read from: {:?}", project_file_path);
    let content = fs::read_to_string(&project_file_path).map_err(|e| {
        println!("Error reading file: {}", e);
        e.to_string()
    })?;

    println!("File content length: {} bytes", content.len());
    let data: ProjectData = serde_json::from_str(&content).map_err(|e| {
        println!("Error parsing JSON: {}", e);
        e.to_string()
    })?;

    println!("Successfully loaded project data");
    Ok(data)
}

#[tauri::command]
fn get_project_file_path_debug() -> Result<String, String> {
    let project_file_path = get_project_file_path()?;
    Ok(format!("Using file path: {}", project_file_path.display()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_available_ports_cmd,
            save_project_data,
            load_project_data,
            get_project_file_path_debug
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
