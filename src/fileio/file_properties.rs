use std::fs::{File, Permissions};

//Gets the permissions stuct by using a reference pointer to file
pub fn get_file_permissions(file: &File) -> Permissions {
    //Gete the meta data and unwrwap the result
    let file_metadata = file.metadata();

    //Create the permissions struct as mutable to allow changes to be made
    //return the Permissions struct
    file_metadata.unwrap().permissions()
}

//function to set file permissions to read only
pub fn set_readonly(file: File, mut file_permissions: Permissions) {
    //set readonly to true
    file_permissions.set_readonly(true);

    //apply permissions to file
    file.set_permissions(file_permissions)
        .expect("Error setting file permissions");
}
