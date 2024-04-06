// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn show_path(path: &str) -> String {
    println!("hello from tauri {path}");

    format!(
        "Hello! Your data at {} is being made into a file, generated in Rust!",
        path
    )
}

#[tauri::command(rename_all = "snake_case")]
fn generate_file(data_path: &str) -> () {
    // this call is not not working yet? Why?
    println!("This is just a log message from Rust for now. Selected data_Path is {data_path}");

    // ls -a
    match fs::read_dir(data_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_path, generate_file])
        .run(tauri::generate_context!())
        .expect("error while running  tauri application");
}
