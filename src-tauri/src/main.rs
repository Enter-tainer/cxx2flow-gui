// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cxx2flow::display::{dot::Dot, GraphDisplayBackend};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_graphviz(code: String) -> Result<String, String> {
    let res = cxx2flow::generate(
        code.as_bytes(),
        "",
        None,
        GraphDisplayBackend::Dot(Dot::new(true)),
    );
    res.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_graphviz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
