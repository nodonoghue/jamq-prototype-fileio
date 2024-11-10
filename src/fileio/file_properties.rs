use std::fs::{File, Permissions};

//Gets the permissions struct by using a reference pointer to file
pub fn get_file_permissions(file: &File) -> Permissions {
    //Get the metadata and unwrap the result
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
    match file.set_permissions(file_permissions) {
        Ok(()) => (),
        Err(error) => panic!("Error setting file permissions:  {error:?}"),
    };
}
