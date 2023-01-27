use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Configs {
    output_directory: String,
}

//open the file from the path argument
pub fn open_file(path: String) -> File {
    println!("filename: {path}");
    //Open the file
    let file_open_result = File::options().append(true).open(&path);

    //Unwraps the open result to get the file
    //returns the file struct
    match file_open_result {
        Ok(file) => file,
        Err(error) => panic!("Error opening file at path {path:?}.  {error:?}"),
    }
}

//get the output directory from settings.json
pub fn get_output_directory() -> String {
    let file_name: String = String::from("settings.json");
    //Read the file
    let contents = fs::read_to_string(file_name).expect("Error reading file!");

    //Convert from String to &str
    let contents_str: &str = &contents[..];

    //deserialize into a struct
    let configs: Configs = match serde_json::from_str(contents_str) {
        Ok(configs) => configs,
        Err(error) => panic!("Error reading configuration file:  {error:?}"),
    };

    //return the output_directory setting
    configs.output_directory
}
