use std::{collections::HashSet, usize};

use crate::utils;

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const OBSTACLE: char = '#';
const BLANK: char = '.';

fn print_grid(grid: &Vec<Vec<char>>) {
    let mut s = String::new();

    for l in grid {
        s += &l.into_iter().collect::<String>();
        s += "\n";
    }

    println!("{s}");
}

fn get_grid() -> (Vec<Vec<char>>, i32) {
    let file_content = utils::read_file_to_u8("./inputs/6");

    let mut grid: Vec<Vec<char>> = vec![];

    let mut i = 0;

    let mut initial_position: i32 = -1;

    let mut v = vec![];

    while i < file_content.len() {
        if file_content[i] == b'\n' {
            grid.push(v.clone());
            v = vec![];
            i += 1;
            continue;
        }

        if file_content[i] as char == '^' {
            initial_position = i as i32 - grid.len() as i32;
        }

        v.push(file_content[i] as char);

        i += 1;
    }

    return (grid, initial_position);
}

pub fn day6p1() {
    let (mut grid, initial_position) = get_grid();

    let width = grid[0].len();

    let initial_row = initial_position / width as i32;
    let initial_col = initial_position % width as i32;

    let mut total_cells_visited = 0;

    let mut r = initial_row as usize;
    let mut c = initial_col as usize;

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    loop {
        match set.get(&(r, c)) {
            Some(_) => {}

            None => {
                total_cells_visited += 1;
                set.insert((r, c));
            }
        }

        match grid[r][c] {
            UP => {
                if r == 0 {
                    break;
                }

                if grid[r - 1][c] == OBSTACLE {
                    grid[r][c] = RIGHT;
                    continue;
                }

                grid[r][c] = BLANK;
                grid[r - 1][c] = UP;

                r -= 1;
            }

            DOWN => {
                if r == grid.len() - 1 {
                    break;
                }

                if grid[r + 1][c] == OBSTACLE {
                    grid[r][c] = LEFT;
                    continue;
                }

                grid[r][c] = BLANK;
                grid[r + 1][c] = DOWN;

                r += 1;
            }

            LEFT => {
                if c == 0 {
                    break;
                }

                if grid[r][c - 1] == OBSTACLE {
                    grid[r][c] = UP;
                    continue;
                }

                grid[r][c] = BLANK;
                grid[r][c - 1] = LEFT;

                c -= 1;
            }

            RIGHT => {
                if c == grid[0].len() - 1 {
                    break;
                }

                if grid[r][c + 1] == OBSTACLE {
                    grid[r][c] = DOWN;
                    continue;
                }

                grid[r][c] = BLANK;
                grid[r][c + 1] = RIGHT;

                c += 1;
            }

            _ => {}
        }
    }

    println!("Day6 P1: {}", total_cells_visited);
}

pub fn day6p2() {
    let (mut grid, initial_position) = get_grid();

    let width = grid[0].len();

    let initial_row = (initial_position / width as i32) as usize;
    let initial_col = (initial_position % width as i32) as usize;

    let mut r = initial_row as usize;
    let mut c = initial_col as usize;

    let initial_guard_direction = grid[r][c];

    // (row, pos, direction)
    let mut set: HashSet<(usize, usize, char)> = HashSet::new();

    let mut total_obstacle_to_loop = 0;

    // place an obstacle at every position
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == OBSTACLE || (row == initial_row && col == initial_col) {
                continue;
            }

            // place an obstacle here
            grid[row][col] = OBSTACLE;

            set = HashSet::new();

            r = initial_row;
            c = initial_col;

            grid[initial_row][initial_col] = initial_guard_direction;

            let mut guard_loops = false;

            // simulate if the guard gets stuck or not
            loop {
                let current_cell = grid[r][c];

                match set.get(&(r, c, current_cell)) {
                    Some(_) => {
                        // if we find the guard in the same row, col in the same position
                        // then it's a loop
                        guard_loops = true;
                        break;
                    }

                    None => {
                        set.insert((r, c, current_cell));
                    }
                }

                match current_cell {
                    UP => {
                        if r == 0 {
                            break;
                        }

                        if grid[r - 1][c] == OBSTACLE {
                            grid[r][c] = RIGHT;
                            continue;
                        }

                        grid[r][c] = BLANK;
                        grid[r - 1][c] = UP;

                        r -= 1;
                    }

                    DOWN => {
                        if r == grid.len() - 1 {
                            break;
                        }

                        if grid[r + 1][c] == OBSTACLE {
                            grid[r][c] = LEFT;
                            continue;
                        }

                        grid[r][c] = BLANK;
                        grid[r + 1][c] = DOWN;

                        r += 1;
                    }

                    LEFT => {
                        if c == 0 {
                            break;
                        }

                        if grid[r][c - 1] == OBSTACLE {
                            grid[r][c] = UP;
                            continue;
                        }

                        grid[r][c] = BLANK;
                        grid[r][c - 1] = LEFT;

                        c -= 1;
                    }

                    RIGHT => {
                        if c == grid[0].len() - 1 {
                            break;
                        }

                        if grid[r][c + 1] == OBSTACLE {
                            grid[r][c] = DOWN;
                            continue;
                        }

                        grid[r][c] = BLANK;
                        grid[r][c + 1] = RIGHT;

                        c += 1;
                    }

                    _ => {}
                }
            } // loop ends

            if guard_loops {
                total_obstacle_to_loop += 1;
            }

            grid[row][col] = '.';
        }
    }

    println!("Day6 P2: {}", total_obstacle_to_loop);
}
