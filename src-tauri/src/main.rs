#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn generate_graphviz(code: String) -> Result<String, String> {
    let res = cxx2flow::generate(code.as_bytes(), "", None, true, false);
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_graphviz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
