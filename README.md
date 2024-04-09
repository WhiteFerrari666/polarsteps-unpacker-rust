# Polarsteps Unpacker - Rust edition
Small app that takes the contents of a Polarsteps user data export and converts it into an MS Word document.

This is my project to get acquainted with Rust and, furthermore, with web technologies in the frontend.

#### used frameworks
Tauri/Rust in the Backend and SolidJS/Typescript in the frontend.

#### run dev environment
`npm run tauri dev` -- for anything else look @ Tauri Getting Started examples

#### stuff I learned along the way

##### tauri commands and parameter names 
Tauri converts all parameters used in JS calls to camelCase by default! 

Therefore, if you have a `#[tauri::command]` in Rust whose function parameter name uses snake_case (as is the style recommendation by VS Code or rust-analyzer, for that matter), it is necessary that such `command`s are annotated in the following way: `#[tauri::command(rename_all = "snake_case")]`

Otherwise the invocation simply won't work with no particular error messages by default. An error message will, however, be displayed in the browser console.

See also [this StackOverflow answer](https://stackoverflow.com/a/74633778/16063145).