use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, Permissions};
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Configs {
    output_directory: String,
}

//create a function to wrap calles to the chain of calles required to create the file
pub fn write_jmq_file(filename: String) {
    //get the output path from the configuration file
    let output_directory: String = get_output_directory();

    //Check if the directory exists and create if it does not
    create_directory(&output_directory);

    //get the full file path
    let full_path = get_full_path(filename);

    //Check if the file exists and create if it does not
    create_file(&full_path);

    //open the file to get the file struct
    let mut file = open_file(full_path);

    //write to the file
    file = write_to_file(file);

    //get file permissions struct
    let file_permissions = get_file_permissions(&file);

    //call function to set permissions to readonly
    set_readonly(file, file_permissions);
}

//public function to facilitate getting the full file path to use when creating the jmq file
pub fn get_full_path(filename: String) -> String {
    //get the root of the file path
    let root_path: String = get_output_directory();

    //get the full file path, including file name
    get_path(filename, &root_path)
}

//get the output directory from settings.json
fn get_output_directory() -> String {
    let file_name: String = String::from("settings.json");
    //Read the file
    let contents = fs::read_to_string(file_name).expect("Error reading file!");

    //Convert from String to &str
    let contents_str: &str = &contents[..];

    //deserialize into a struct
    let configs: Configs = serde_json::from_str(contents_str).unwrap();

    //return the output_directory setting
    configs.output_directory
}

//function to return the path string, accempts the timestamp u128
fn get_path(filename: String, output_directory: &String) -> String {
    //set the initial file name to the timestamp
    let mut full_path = output_directory.to_owned();

    //Add the file name
    full_path = full_path + &filename;

    //append the file extension
    full_path.push_str(".jmq");

    //return the path
    full_path
}

//create directory in path argument if it doesn't exist
fn create_directory(path: &String) {
    //check if the path exists, if it does this function does nothing
    if !Path::new(path).is_dir() {
        //if the path does not exist, create it
        fs::create_dir(path).expect("Error creating directory");
    }
}

//create the file from the path argument
fn create_file(path: &String) {
    //if the file already exists, this function does nothing
    if !Path::new(path).is_file() {
        //if the file does not exist, create it
        File::create(path).expect("Error creating file");
    }
}

//open the file from the path argument
fn open_file(path: String) -> File {
    println!("filename: {}", path);
    //Open the file
    let file_open_result = File::options().append(true).open(path);

    //Unwraps the open result to get the file
    //returns the file struct
    file_open_result.unwrap()
}

//Writes to the specified file
fn write_to_file(mut file: File) -> File {
    //Open the file

    //Set the contents to write, currently hard coded for proof of concept
    //TODO:  Refactor later to make the contents an argument passed into the function, not hard coded
    let contents = String::from("Hello, world!");

    //Write to the file, unwrap appears to unwrap the underlying type from the result?
    //Will handle a file write failure with a panic! and message in the terminal
    file.write_all(contents.as_bytes())
        .expect("Error writing to file");

    //return the file
    file
}

//Gets the permissions stuct by using a reference pointer to file
fn get_file_permissions(file: &File) -> Permissions {
    //Gete the meta data and unwrwap the result
    let file_metadata = file.metadata();

    //Create the permissions struct as mutable to allow changes to be made
    //return the Permissions struct
    file_metadata.unwrap().permissions()
}

//function to set file permissions to read only
fn set_readonly(file: File, mut file_permissions: Permissions) {
    //set readonly to true
    file_permissions.set_readonly(true);

    //apply permissions to file
    file.set_permissions(file_permissions)
        .expect("Error setting file permissions");
}
