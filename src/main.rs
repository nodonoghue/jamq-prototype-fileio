use chrono::Utc;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dt = Utc::now();
    let timestamp = dt.timestamp();

    let mut path = timestamp.to_string();
    path.push_str(".log");
    if Path::new(&path).is_file() {
        write_to_file(path);
    } else {
        File::create(&path)?;
        write_to_file(path);
    }

    Ok(())
}

fn write_to_file(path: String) {
    let file = File::options().append(true).open(path);
    let contents = String::from("Hello, world!");
    file.unwrap()
        .write_all(contents.as_bytes())
        .expect("Error writing to file");
}
