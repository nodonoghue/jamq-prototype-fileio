mod file_directory;
mod file_properties;
mod write_file;
mod read_file;

//create a function to wrap calles to the chain of calles required to create the file
pub fn write_jmq_file(filename: String) {
    //get the output path from the configuration file
    let output_directory: String = read_file::get_output_directory();

    //Check if the directory exists and create if it does not
    file_directory::create_directory(&output_directory);

    //get the full file path
    let full_path = file_directory::get_full_path(filename, output_directory);

    //Check if the file exists and create if it does not
    write_file::create_file(&full_path);

    //open the file to get the file struct
    let mut file = read_file::open_file(full_path);

    //write to the file
    file = write_file::write_to_file(file);

    //get file permissions struct
    let file_permissions = file_properties::get_file_permissions(&file);

    //call function to set permissions to readonly
    file_properties::set_readonly(file, file_permissions);
}
