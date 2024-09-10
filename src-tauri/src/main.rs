// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::{c_char, CStr, CString};

use tauri::Runtime;

extern "C" {
    fn raw_convert_each_value_to_uppercase(vec: *mut c_char, lenght: i32) -> *mut c_char;
    fn test_cpp(vec: *mut i32, length: i32) -> i32;
    // fn raw_sum_array_of_numbers(input: Vec<i32>) -> i32;
    // fn raw_convert_each_value_to_uppercase(input: Vec<String>) -> Vec<String>;
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn sum_array_of_numbers<R: Runtime>(
    data: Vec<i32>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<i32, String> {
    // let result = data.iter().sum::<i32>();

    let test = Vec::<&str>::new();
    let a = test.as_ptr();
    let l = test.len();

    // unsafe {
    //     let result = raw_sum_array_of_numbers(data);
    //     println!("sum_array_of_numbers calcs: {}", result);
    //     Ok(result)
    // }
    Ok(8)
}

#[tauri::command]
async fn convert_each_value_to_uppercase<R: Runtime>(
    data: Vec<String>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<Vec<String>, String> {
    let length = data.len();
    let mut c_strings: Vec<CString> = data.into_iter().map(|s| CString::new(s).unwrap()).collect();

    let mut pointers: Vec<*mut c_char> = c_strings
        .iter_mut()
        .map(|s| s.as_ptr() as *mut c_char)
        .collect();
    pointers.push(std::ptr::null_mut());

    let nova_data = pointers.as_mut_ptr();

    unsafe {
        let result = raw_convert_each_value_to_uppercase(nova_data as *mut i8, length as i32);

        let mut result_vec: Vec<String> = Vec::new();

        let mut i = 0;
        loop {
            let current_str = *result.offset(i);
            if current_str.is_null() {
                break;
            }

            let c_str = CString::from_raw(current_str);
            let str_slice = c_str.to_str().unwrap();
            result_vec.push(str_slice.to_string());
            i += 1;
        }

        Ok(result_vec)
    }

    // let mut result: Vec<String> = Vec::new();
    // for word in data {
    //     result.push(word.to_uppercase());
    // }
    // Ok(result)

    // unsafe {
    //     let result = raw_convert_each_value_to_uppercase(data);
    //     Ok(result)
    // }
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
