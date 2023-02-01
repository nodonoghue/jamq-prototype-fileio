use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

//use super::file_directory;

#[derive(Serialize, Deserialize)]
struct Configs {
    output_directory: String,
}

//open the file from the path argument
pub fn open_file(path: String) -> File {
    println!("filename: {path}");
    //Open the file
    let file_open_result = File::options().append(true).open(path);

    //Unwraps the open result to get the file
    //returns the file struct
    match file_open_result{
        Ok(file) => file,
        Err(error) => panic!("File read error.  {error:?}")
    }    
}

//get the output directory from settings.json
pub fn get_output_directory() -> String {
    let file_name: String = String::from("settings.json");
    //Read the file
    match fs::read_to_string(file_name)
    {
        Ok(contents) => {
            let configs = get_configs(contents);
            configs.output_directory
        },
        Err(error)=> panic!("Error reading config file.  {error:?}")  
    }
}

fn get_configs(file_contents: String) -> Configs
{
    let contents_str: &str = &file_contents[..];

    //deserialize into a struct    
    let configs: Configs = serde_json::from_str(contents_str).expect("Error reading configs from file.");
    configs
}