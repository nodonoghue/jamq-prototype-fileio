use std::fs::{File, Permissions};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    //Tried another way of getting the current timestamp, as UNIX Epoc.
    //This time using the std::time lib and in milliseconds.
    //I could have done the same by importing chrono as well.
    //I wanted to stick with std libs for my learning prototypes.
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    //Create the file name, could also be the full path to the file.
    //let mut path = timestamp.to_string();
    let mut path = time.to_string();

    //Changed the file extension to see what happens.  More inline with potential project design
    path.push_str(".jmq");

    //check if the file exists
    if Path::new(&path).is_file() {
        //If it exists already, unlikely with update to naming process, write to the existing file
        write_to_file(path);
    } else {
        //Otherwise create the file, then write to the new file.
        File::create(&path)?;
        write_to_file(path);
    }

    Ok(())
}

//Writes to the specified file
fn write_to_file(path: String) {
    //Open the file
    let file_open_result = File::options().append(true).open(path);
    let mut file = file_open_result.unwrap();

    //Set the contents to write, currently hard coded for proof of concept
    //TODO:  Refactor later to make the contents an argument passed into the function, not hard coded
    let contents = String::from("Hello, world!");

    //Write to the file, unwrap appears to unwrap the underlying type from the result?
    //Will handle a file write failure with a panic! and message in the terminal
    file.write_all(contents.as_bytes())
        .expect("Error writing to file");

    //Get the file permissions set to read_only
    //Found to avoid fighting with the borrow checker I would pass a reference of file
    //to my function, then return the permissions struct to use for setting to readonly
    let file_permissions = set_readoonly(&file);

    //Apply updated permissions to file
    file.set_permissions(file_permissions)
        .expect("Error setting file permissions");
}

//Gets the permissions stuct by using a reference pointer to file
fn set_readoonly(file: &File) -> Permissions {
    //Gete the meta data and unwrwap the result
    let file_metadata = file.metadata();

    //Create the permissions struct as mutable to allow changes to be made
    let mut file_permissions = file_metadata.unwrap().permissions();

    //set readonly to true
    file_permissions.set_readonly(true);

    //return the permissions struct
    file_permissions
}
