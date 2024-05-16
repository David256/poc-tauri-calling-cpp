// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use tauri::Runtime;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn sum_array_of_numbers<R: Runtime>(
    data: Vec<i32>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<i32, String> {
    let result = data.iter().sum::<i32>();

    println!("sum_array_of_numbers calcs: {}", result);

    Ok(result)
}

#[tauri::command]
async fn convert_each_value_to_uppercase<R: Runtime>(
    data: Vec<String>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();
    for word in data {
        result.push(word.to_uppercase());
    }
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sum_array_of_numbers,
            convert_each_value_to_uppercase
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
