use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Map {
    Empty,
    Wall,
    End,
    Start,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Map::Empty => ".",
                Map::Wall => "#",
                Map::End => "E",
                Map::Start => "S",
            }
        )
    }
}


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

pub fn is_in_bounds<T>(map: &Vec<Vec<T>>, row: i32, col: i32) -> bool {
    return row >= 0 && col >= 0 && row < map.len() as i32 && col < map[0].len() as i32;
}

pub fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for c in row {
            print!("{}", c)
        }

        println!();
    }
}
