use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    u64, usize,
};

use crate::utils;

enum Map {
    Emtpy,
    Wall,
    Start,
    End,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
enum Dir {
    Top,
    Right,
    Down,
    Left,
}

impl Dir {
    fn opposite(&self) -> Self {
        use Dir::*;

        match self {
            Dir::Top => Down,
            Dir::Right => Left,
            Dir::Down => Top,
            Dir::Left => Right,
        }
    }

    fn cost(&self, other: Dir) -> u64 {
        use Dir::*;

        match self {
            Dir::Top => match other {
                Top => 1,
                Right | Left => 1001,
                Down => u64::MAX,
            },

            Dir::Down => match other {
                Top => u64::MAX,
                Right | Left => 1001,
                Down => 1,
            },

            Dir::Left => match other {
                Top | Down => 1001,
                Right => u64::MAX,
                Left => 1,
            },

            Dir::Right => match other {
                Top | Down => 1001,
                Right => 1,
                Left => u64::MAX,
            },
        }
    }
}

fn is_in_bounds<T: Sized>(map: &Vec<Vec<T>>, row: i32, col: i32) -> bool {
    return row >= 0 && col >= 0 && row < map.len() as i32 && col < map[0].len() as i32;
}

fn parse_input() -> (Vec<Vec<Map>>, (usize, usize)) {
    let file_contents = utils::read_file_to_string("./inputs/16");

    let mut map: Vec<Vec<Map>> = vec![];

    let (mut row, mut col) = (0, 0);

    for (r, line) in file_contents.lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(c, x)| match x {
                    '#' => Map::Wall,
                    'S' => {
                        row = r;
                        col = c;

                        Map::Start
                    }
                    'E' => Map::End,
                    _ => Map::Emtpy,
                })
                .collect(),
        )
    }

    (map, (row, col))
}

type Cost = u64;
type MyHeap = BinaryHeap<Reverse<(Cost, usize, usize, Dir)>>;
type Set = HashSet<(usize, usize, Dir)>;

fn bfs(map: &Vec<Vec<Map>>, start_row: usize, start_col: usize) -> u64 {
    // State: (total_cost, row, col, current_direction)
    let mut heap: MyHeap = BinaryHeap::new();
    heap.push(Reverse((0, start_row, start_col, Dir::Right)));

    // Track minimum cost to reach each cell with a specific direction
    let mut costs = vec![vec![vec![u64::MAX; 4]; map[0].len()]; map.len()];

    let directions = [
        (-1, 0, Dir::Top),
        (0, 1, Dir::Right),
        (1, 0, Dir::Down),
        (0, -1, Dir::Left),
    ];

    while let Some(Reverse((current_cost, row, col, current_dir))) = heap.pop() {
        // Check if we've reached the end
        if matches!(map[row][col], Map::End) {
            return current_cost;
        }

        // Skip if we've found a cheaper path to this cell with this direction
        if current_cost >= costs[row][col][current_dir as usize] {
            continue;
        }
        costs[row][col][current_dir as usize] = current_cost;

        for (dr, dc, new_dir) in directions.iter() {
            if *new_dir == current_dir.opposite() {
                continue;
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if is_in_bounds(map, new_row, new_col) {
                let (new_row, new_col) = (new_row as usize, new_col as usize);

                if matches!(map[new_row][new_col], Map::Wall) {
                    continue;
                }

                let move_cost = current_dir.cost(*new_dir);
                let new_total_cost = current_cost + move_cost;

                // Add to heap if this is potentially a better path
                // Don't actually need to check this, but it's faster if we do
                if new_total_cost < costs[new_row][new_col][*new_dir as usize] {
                    heap.push(Reverse((new_total_cost, new_row, new_col, *new_dir)));
                }
            }
        }
    }

    // couldn't find a path
    u64::MAX
}

// 128428 -> Too high
// 102504 -> correct
pub fn day16p1() {
    let (map, (start_row, start_col)) = parse_input();

    let cost = bfs(&map, start_row, start_col);

    println!("Day16 P1: {}", cost);
}
