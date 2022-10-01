use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    //Create the file name, could also be the full path to the file.
    //let mut path = timestamp.to_string();
    let mut path = time.to_string();
    path.push_str(".log");

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
    let file = File::options().append(true).open(path);

    //Set the contents to write, currently hard coded for proof of concept
    //TODO:  Refactor later to make the contents an argument passed into the function, not hard coded
    let contents = String::from("Hello, world!");

    //Write to the file, unwrap appears to unwrap the underlying type from the result?
    //Will handle a file write failure with, I believe, a panic! and message
    //TODO:  Learn more about the specifics of unwrap()
    file.unwrap()
        .write_all(contents.as_bytes())
        .expect("Error writing to file");
}
