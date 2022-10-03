use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, Permissions};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Configs {
    output_directory: String,
}

fn main() -> std::io::Result<()> {
    //Tried another way of getting the current timestamp, as UNIX Epoc.
    //This time using the std::time lib and in milliseconds.
    //I could have done the same by importing chrono as well.
    //I wanted to stick with std libs for my learning prototypes.

    //read the settings file
    let output_directory: String = get_output_directory();

    //Get current timestamp in milliseconds
    let timestamp = get_current_timestamp_millis();

    //Create the file name, could also be the full path to the file.
    let full_path = get_path(timestamp, &output_directory);

    //call file io wrapper function
    perform_file_io(&full_path, &output_directory);

    Ok(())
}

//function to get the current timestamp as millis
fn get_current_timestamp_millis() -> u128 {
    //uses std lib to get the current timestamp as a diration since epoch
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

//function to return the path string, accempts the timestamp u128
fn get_path(timestamp: u128, output_directory: &String) -> String {
    //set the initial file name to the timestamp
    let mut path = output_directory.to_owned();

    //Add the file name
    path = path + &timestamp.to_string();

    //append the file extension
    path.push_str(".jmq");

    //return the path
    path
}

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

//wrap calls to all file io functions in a single function call
//takes only a reference to the path variable as an argumentc
fn perform_file_io(path: &String, output_directory: &String) {
    //Check if the directory exists and create if it does not
    create_directory(output_directory);

    //Check if the file exists and create if it does not
    create_file(path);

    //open the file to get the file struct
    let mut file = open_file(path);

    //write to the file
    file = write_to_file(file);

    //get file permissions struct
    let file_permissions = get_file_permissions(&file);

    //call function to set permissions to readonly
    set_readonly(file, file_permissions);
}

fn create_directory(path: &String) {
    if !Path::new(path).is_dir() {
        fs::create_dir(path).expect("Error creating directory");
    }
}

fn create_file(path: &String) {
    if !Path::new(path).is_file() {
        File::create(path).expect("Error creating file");
    }
}

fn open_file(path: &String) -> File {
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
