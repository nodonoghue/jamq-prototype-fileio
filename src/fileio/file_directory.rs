use std::fs;
use std::path::Path;

pub fn get_full_path(filename: String, root_path: String) -> String {
    //get the full file path, including file name
    get_path(filename, &root_path)
}

//function to return the path string, accepts the timestamp u128
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
pub fn create_directory(path: &String) {
    //check if the path exists, if it does this function does nothing
    if !Path::new(path).is_dir() {
        //if the path does not exist, create it
        match fs::create_dir(path){
            Ok(()) => (),
            Err(error) => panic!("Error creating output directory.  {error:?}")
        }
    }
}
