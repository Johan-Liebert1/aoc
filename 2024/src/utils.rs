use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn read_file_to_string(path: &str) -> String {
    let file = File::open(path);

    if !file.is_ok() {
        eprintln!("Failed to open file '{}' with error: {:?}", path, file);
        exit(1);
    }

    let mut file = file.unwrap();
    let mut file_contents = String::new();

    let _ = file.read_to_string(&mut file_contents);

    return file_contents;
}

pub fn read_file_to_u8(path: &str) -> Vec<u8> {
    let file = File::open(path);

    if !file.is_ok() {
        eprintln!("Failed to open file '{}' with error: {:?}", path, file);
        exit(1);
    }

    let mut file = file.unwrap();
    let mut file_contents = Vec::new();

    let _ = file.read_to_end(&mut file_contents);

    return file_contents;
}
