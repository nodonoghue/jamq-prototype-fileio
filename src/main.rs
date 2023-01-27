use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

//declare modules
mod fileio;

fn main() -> std::io::Result<()> {
    //Get current timestamp in milliseconds
    let filename = get_current_timestamp_millis();

    //call file io wrapper function
    fileio::write_jmq_file(filename);

    Ok(())
}

//function to get the current timestamp as millis
fn get_current_timestamp_millis() -> String {
    //uses std lib to get the current timestamp as a diration since epoch
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 500))
        .as_millis()
        .to_string()
}
