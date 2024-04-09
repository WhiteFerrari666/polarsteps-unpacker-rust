## Small app that prints JSON content from Polarsteps into an MS Word document

#### run dev environment
`npm run tauri dev` -- for anything else look @ Tauri Getting Started examples

#### stuff I learned along the way

##### tauri commands and parameter names 
Tauri converts all parameters used in JS calls to camelCase by default! 

Therefore, if you have a `#[tauri::command]` in Rust whose function parameter name uses snake_case (as is the style recommendation by VS Code or rust-analyzer, for that matter), it is necessary that such `command`s are annotated in the following way: `#[tauri::command(rename_all = "snake_case")]`

Otherwise the invocation simply won't work with no particular error messages by default. An error message will, however, be displayed in the browser console.

See also [this StackOverflow answer](https://stackoverflow.com/a/74633778/16063145).