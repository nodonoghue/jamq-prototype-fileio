use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//create the file from the path argument
pub fn create_file(path: &String) {
    //if the file already exists, this function does nothing
    if !Path::new(path).is_file() {
        //if the file does not exist, create it
        File::create(path).expect("Error creating file");
    }
}
//Writes to the specified file
pub fn write_to_file(mut file: File) -> File {
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
