use std::{
    fmt::{Debug, Display},
    usize,
};

use crate::utils;

type Grid = Vec<Vec<char>>;
type Cache = Vec<Vec<bool>>;

#[derive(Clone, Copy, PartialEq)]
enum CameFrom {
    Left,
    Right,
    Up,
    Down,
    First,
}

impl CameFrom {
    fn opposite(&self) -> Self {
        match self {
            CameFrom::Left => CameFrom::Right,
            CameFrom::Right => CameFrom::Left,
            CameFrom::Up => CameFrom::Down,
            CameFrom::Down => CameFrom::Up,
            CameFrom::First => CameFrom::First,
        }
    }
}

impl Debug for CameFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            CameFrom::Left => "Left",
            CameFrom::Right => "Right",
            CameFrom::Up => "Up",
            CameFrom::Down => "Down",
            CameFrom::First => "First",
        };

        write!(f, "{}", x)
    }
}

impl Display for CameFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const DIRECTIONS: [(i32, i32, CameFrom); 4] = [
    (-1, 0, CameFrom::Down),  // up
    (1, 0, CameFrom::Up),     // down
    (0, -1, CameFrom::Right), // left
    (0, 1, CameFrom::Left),   // right
];

fn is_in_bounds(grid: &Grid, row: i32, col: i32) -> bool {
    return row >= 0 && col >= 0 && row < grid.len() as i32 && col < grid[0].len() as i32;
}

fn get_perimeter(grid: &Grid, visited: &mut Cache, row: usize, col: usize) -> (i32, i32) {
    use CameFrom::*;

    let mut perimeter = 0;
    let mut area = 0;

    let mut queue: Vec<(i32, i32, CameFrom)> = vec![(row as i32, col as i32, First)];

    while queue.len() > 0 {
        let (c_row, c_col, c_came_from) = queue[0];

        queue = queue[1..].to_vec();

        if visited[c_row as usize][c_col as usize] {
            continue;
        }

        visited[c_row as usize][c_col as usize] = true;

        area += 1;

        match c_came_from {
            First => {
                perimeter += 4;
            }

            _ => {
                let mut total_connected_neighbors = 0;

                for (ra, ca, _) in DIRECTIONS {
                    let (nrow, ncol) = (c_row + ra, c_col + ca);

                    if is_in_bounds(grid, nrow, ncol)
                        && visited[nrow as usize][ncol as usize]
                        && grid[nrow as usize][ncol as usize]
                            == grid[c_row as usize][c_col as usize]
                    {
                        total_connected_neighbors += 1;
                    }
                }

                if total_connected_neighbors == 1 {
                    perimeter += 2;
                } else if total_connected_neighbors == 3 {
                    perimeter -= 2;
                } else if total_connected_neighbors == 4 {
                    perimeter -= 4;
                }
            }
        };

        for (d_row, d_col, d_came_from) in DIRECTIONS {
            let (n_row, n_col) = (c_row + d_row, c_col + d_col);

            if is_in_bounds(grid, n_row, n_col)
                && grid[n_row as usize][n_col as usize] == grid[row][col]
            {
                if visited[n_row as usize][n_col as usize] {
                    continue;
                }

                queue.push((n_row, n_col, d_came_from));
            }
        }
    }

    return (perimeter, area);
}

pub fn day12p1() {
    let file_contents = utils::read_file_to_string("./inputs/12");

    let mut grid: Grid = vec![];

    for line in file_contents.lines() {
        grid.push(line.chars().collect());
    }

    let mut visited: Cache = vec![vec![false; grid[0].len()]; grid.len()];

    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !visited[row][col] {
                let (perimeter, area) = get_perimeter(&grid, &mut visited, row, col);
                total += area * perimeter;
            }
        }
    }

    println!("Day 12 P1: {total}");
}
