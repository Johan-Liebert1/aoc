use std::{fmt::Display, usize};

use crate::utils;

const ROWS: usize = 71;
const COLS: usize = 71;
const CORRUPTED_MEMORY: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Map {
    Emtpy,
    Wall,
    End,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Emtpy => ".",
            Wall => "#",
            End => "E",
        };

        write!(f, "{}", v)
    }
}

use Map::*;

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn parse_input() -> Vec<Vec<usize>> {
    let file = utils::read_file_to_string("./inputs/18");

    let mut v = vec![];

    for line in file.lines() {
        let mut x = line
            .split(",")
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        x.reverse();

        v.push(x);
    }

    return v;
}

// since this is unweighted we can do this with simple bfs
fn bfs(grid: &Vec<Vec<Map>>) -> usize {
    let mut all_costs = vec![vec![usize::MAX; ROWS]; COLS];

    let mut queue: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];

    while queue.len() > 0 {
        let (row, col, current_cost) = queue[0];
        queue = queue[1..].to_vec();

        if grid[row][col] == End {
            println!("for final: {current_cost}");
        }

        if all_costs[row][col] <= current_cost {
            continue;
        }

        all_costs[row][col] = current_cost;

        for (dr, dc) in DIRS {
            let (new_row, new_col) = (row as i32 + dr, col as i32 + dc);

            if utils::is_in_bounds(grid, new_row, new_col) && grid[row][col] != Wall {
                let (new_row, new_col) = (new_row as usize, new_col as usize);

                if all_costs[new_row][new_col] > current_cost + 1 {
                    queue.push((new_row, new_col, current_cost + 1));
                }
            }
        }
    }

    return all_costs[ROWS - 1][COLS - 1];
}

pub fn part1() {
    let ffed_coordinates = parse_input();

    let mut grid: Vec<Vec<Map>> = vec![vec![Emtpy; ROWS]; COLS];

    for coord in &ffed_coordinates[..CORRUPTED_MEMORY] {
        grid[coord[0]][coord[1]] = Map::Wall;
    }

    grid[ROWS - 1][COLS - 1] = End;

    utils::print_grid(&grid);

    let min_cost = bfs(&grid);

    println!("Day18 P1: {min_cost}");
}

pub fn part2() {
    let ffed_coordinates = parse_input();

    let mut grid: Vec<Vec<Map>> = vec![vec![Emtpy; ROWS]; COLS];

    let (mut low, mut high) = (0, ffed_coordinates.len() - 1);

    let mut last_unreachable = 0;

    grid[ROWS - 1][COLS - 1] = End;

    while low <= high {
        let mid = low + (high - low) / 2;

        grid = vec![vec![Emtpy; ROWS]; COLS];

        for coord in &ffed_coordinates[..mid] {
            grid[coord[0]][coord[1]] = Map::Wall;
        }

        let min_cost = bfs(&grid);

        if min_cost != usize::MAX {
            // path is not blocked
            low = mid + 1;
        } else {
            high = mid - 1;
            last_unreachable = mid;
        }
    }

    let mut coord = ffed_coordinates[last_unreachable].clone();
    coord.reverse();

    println!(
        "Day18 P2: Last unreachable: {last_unreachable}. Coordinate: {:?}",
        coord
    );
}
