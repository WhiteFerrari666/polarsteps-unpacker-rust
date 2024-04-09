// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;

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

#[derive(Serialize, Deserialize, Debug)]
struct Trip {
    name: String,
    start_date: i32,
    end_date: i32,
    total_km: i32,
    summary: String,
    cover_photo_path: String,
    all_steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Step {
    id: i32,
    description: String,
    slug: String,
    start_time: i32,
    location: StepLocation,
    weather_condition: String,
    weather_temperatur: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct StepLocation {
    name: String,
    detail: String,
    lat: i32,
    lon: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foos {
    baz: Vec<Foo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    bar: String,
}

#[tauri::command(rename_all = "snake_case")]
fn generate_file_from_json(data_path: &str) -> () {
    println!("Trying to generate trip document from {data_path}!");

    let step_location = StepLocation {
        name: "jooo".to_string(),
        detail: "detaillll".to_string(),
        lat: 1,
        lon: 2,
    };

    let serialized = serde_json::to_string(&step_location).unwrap();
    println!("serialized: {}", serialized);

    let mut vec = Vec::new();
    vec.push(Foo {
        bar: "eioio".to_string(),
    });
    vec.push(Foo {
        bar: "abababa".to_string(),
    });

    let my_foos = Foos { baz: vec };

    let serialized = serde_json::to_string(&my_foos).unwrap();

    println!("serialized: {}", serialized)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_path,
            generate_file,
            generate_file_from_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running  tauri application");
}
