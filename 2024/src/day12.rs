use std::{
    fmt::{Debug, Display},
    usize,
};

use crate::utils::{self, is_in_bounds};

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
    (-1, 0, CameFrom::Up),   // up
    (1, 0, CameFrom::Down),  // down
    (0, -1, CameFrom::Left), // left
    (0, 1, CameFrom::Right), // right
];

fn is_neighbor(grid: &Grid, row: i32, col: i32, nrow: i32, ncol: i32) -> bool {
    return is_in_bounds(grid, row, col)
        && is_in_bounds(grid, nrow, ncol)
        && grid[row as usize][col as usize] == grid[nrow as usize][ncol as usize];
}

fn get_grid() -> (Grid, Cache) {
    let file_contents = utils::read_file_to_string("./inputs/12");

    let mut grid: Grid = vec![];

    for line in file_contents.lines() {
        grid.push(line.chars().collect());
    }

    let visited: Cache = vec![vec![false; grid[0].len()]; grid.len()];

    return (grid, visited);
}

fn get_perimeter(
    grid: &Grid,
    visited: &mut Cache,
    row: usize,
    col: usize,
    with_discount: bool,
) -> (i32, i32) {
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
                if !with_discount {
                    let mut total_connected_neighbors = 0;

                    for (ra, ca, _) in DIRECTIONS {
                        let (nrow, ncol) = (c_row + ra, c_col + ca);

                        if is_neighbor(grid, c_row, c_col, nrow, ncol)
                            && visited[nrow as usize][ncol as usize]
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
                } else {
                    for (ra, ca, neighbor_dir) in DIRECTIONS {
                        let (nrow, ncol) = (c_row + ra, c_col + ca);

                        if is_neighbor(grid, c_row, c_col, nrow, ncol)
                            && visited[nrow as usize][ncol as usize]
                        {
                            match neighbor_dir {
                                Left => todo!(),

                                Right => todo!(),

                                Up => todo!(),

                                Down => todo!(),

                                First => todo!(),
                            }
                        }
                    }
                }
            }
        };

        for (d_row, d_col, d_came_from) in DIRECTIONS {
            let (nrow, ncol) = (c_row + d_row, c_col + d_col);

            if is_neighbor(grid, c_row, c_col, nrow, ncol) {
                if visited[nrow as usize][ncol as usize] {
                    continue;
                }

                queue.push((nrow, ncol, d_came_from.opposite()));
            }
        }
    }

    return (perimeter, area);
}

pub fn day12p1() {
    let (grid, mut visited) = get_grid();

    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !visited[row][col] {
                let (perimeter, area) = get_perimeter(&grid, &mut visited, row, col, false);
                total += area * perimeter;
            }
        }
    }

    println!("Day 12 P1: {total}");
}

pub fn day12p2() {
    let (grid, mut visited) = get_grid();

    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !visited[row][col] {
                let (perimeter, area) = get_perimeter(&grid, &mut visited, row, col, true);
                total += area * perimeter;
            }
        }
    }

    println!("Day 12 P1: {total}");
}
