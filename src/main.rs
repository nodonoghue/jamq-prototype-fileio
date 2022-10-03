use std::fs::{File, Permissions};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    //Tried another way of getting the current timestamp, as UNIX Epoc.
    //This time using the std::time lib and in milliseconds.
    //I could have done the same by importing chrono as well.
    //I wanted to stick with std libs for my learning prototypes.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    //Create the file name, could also be the full path to the file.
    let mut path = timestamp.to_string();

    //Changed the file extension to see what happens.  More inline with potential project design
    path.push_str(".jmq");

    //if a file does not exist, creat ie
    if !Path::new(&path).is_file() {
        //create the file since it didn't exist
        File::create(&path)?;
    }

    //call file io wrapper function
    perform_file_io(&path);

    Ok(())
}

//wrap calls to all file io functions in a single function call
//takes only a reference to the path variable as an argument
fn perform_file_io(path: &String) {
    //open the file to get the file struct
    let mut file = open_file(path);

    //write to the file
    file = write_to_file(file);

    //get file permissions struct
    let file_permissions = get_file_permissions(&file);

    //call function to set permissions to readonly
    set_readonly(file, file_permissions);
}

fn open_file(path: &String) -> File {
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
