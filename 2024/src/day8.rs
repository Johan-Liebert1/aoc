use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use crate::utils;

type Map = HashMap<char, Vec<(usize, usize)>>;

fn get_grid() -> (Vec<Vec<char>>, Map) {
    let file_content = utils::read_file_to_u8("./inputs/8");

    let mut grid: Vec<Vec<char>> = vec![];

    for line in file_content.lines() {
        grid.push(line.unwrap().chars().collect());
    }

    let mut map: Map = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let current_char = grid[row][col];

            if current_char.is_ascii_digit() || current_char.is_ascii_alphabetic() {
                match map.get_mut(&current_char) {
                    Some(vector) => {
                        vector.push((row, col));
                    }

                    None => {
                        map.insert(current_char, vec![(row, col)]);
                    }
                }
            }
        }
    }

    return (grid, map);
}

pub fn day8p1() {
    let (grid, map) = get_grid();

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for (_, value) in map {
        // n * n
        for i in 0..value.len() {
            for j in 0..value.len() {
                if i == j {
                    // no antinodes with itself
                    continue;
                }

                let rowi = value[i].0;
                let rowj = value[j].0;
                let coli = value[i].1;
                let colj = value[j].1;

                let abs_row = rowi.abs_diff(rowj);
                let abs_col = coli.abs_diff(colj);

                let row_adder: i32 = if rowi < rowj { -1 } else { 1 };

                let col_adder: i32 = if coli < colj { -1 } else { 1 };

                let final_row = rowi as i32 + (row_adder * abs_row as i32);
                let final_col = coli as i32 + (col_adder * abs_col as i32);

                if final_row >= 0
                    && final_row < grid.len() as i32
                    && final_col >= 0
                    && final_col < grid[0].len() as i32
                {
                    set.insert((final_row, final_col));
                }
            }
        }
    }

    println!("Day8 P1: {}", set.len());
}

pub fn day8p2() {
    let (mut grid, map) = get_grid();

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for (_, value) in &map {
        // n * n
        for i in 0..value.len() {
            for j in 0..value.len() {
                if i == j {
                    // no antinodes with itself
                    continue;
                }

                let rowi = value[i].0;
                let rowj = value[j].0;
                let coli = value[i].1;
                let colj = value[j].1;

                let abs_row = rowi.abs_diff(rowj);
                let abs_col = coli.abs_diff(colj);

                let row_adder: i32 = if rowi < rowj {
                    -1 * abs_row as i32
                } else {
                    1 * abs_row as i32
                };

                let col_adder: i32 = if coli < colj {
                    -1 * abs_col as i32
                } else {
                    1 * abs_col as i32
                };

                let mut final_row = rowi as i32 + (row_adder);
                let mut final_col = coli as i32 + (col_adder);

                while final_row >= 0
                    && final_row < grid.len() as i32
                    && final_col >= 0
                    && final_col < grid[0].len() as i32
                {
                    set.insert((final_row, final_col));

                    final_row = final_row + (row_adder);
                    final_col = final_col + (col_adder);
                }
            }
        }
    }

    for (row, col) in &set {
        grid[*row as usize][*col as usize] = '#';
    }

    for row in grid {
        for r in row {
            print!("{r}");
        }

        println!("");
    }

    let mut final_addition = 0;

    for (_, v) in map {
        if v.len() == 1 {
            continue;
        }

        for (row, col) in v {
            if set.get(&(row as i32, col as i32)).is_none() {
                final_addition += 1;
            }
        }
    }

    println!("Day8 P1: {}", set.len() + final_addition);
}
