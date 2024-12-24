use std::{collections::HashMap, usize, vec};

use crate::utils::{self, Map};

use Map::*;

type Row = usize;
type Col = usize;

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn parse_input() -> (Vec<Vec<Map>>, (Row, Col), (Row, Col)) {
    let file = utils::read_file_to_string("./inputs/20");

    let (mut sr, mut sc) = (0, 0);
    let (mut er, mut ec) = (0, 0);

    let v: Vec<Vec<Map>> = file
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| match char {
                    '#' => Wall,
                    '.' => Empty,

                    'S' => {
                        sr = row;
                        sc = col;

                        Start
                    }

                    'E' => {
                        er = row;
                        ec = col;

                        End
                    }

                    _ => todo!(),
                })
                .collect()
        })
        .collect();

    return (v, (sr, sc), (er, ec));
}

fn bfs(
    map: &Vec<Vec<Map>>,
    start_row: Row,
    start_col: Col,
    end_row: Row,
    end_col: Col,
    abs_min: usize,
) -> usize {
    let mut queue: Vec<(Row, Col, usize)> = vec![(start_row, start_col, 0)];

    let mut visited: Vec<Vec<usize>> = vec![vec![usize::MAX; map[0].len()]; map.len()];

    while let Some(&(row, col, cost)) = queue.first() {
        queue = queue[1..].to_vec();

        if visited[row][col] < cost {
            continue;
        }

        if cost >= abs_min || (cost > abs_min && cost - abs_min > 100) {
            continue;
        }

        visited[row][col] = cost;

        if matches!(map[row][col], End) {
            break;
        }

        for (dr, dc) in DIRS {
            let (new_row, new_col) = (row as i32 + dr, col as i32 + dc);

            if utils::is_in_bounds(map, new_row, new_col) {
                let (new_row, new_col) = (new_row as usize, new_col as usize);

                // only visit the neighbor if it'll decrease our path
                if visited[new_row][new_col] > cost + 1 && !matches!(map[new_row][new_col], Wall) {
                    queue.push((new_row, new_col, cost + 1));
                }
            }
        }
    }

    return visited[end_row][end_col];
}

pub fn part1() {
    let (mut map, (sr, sc), (er, ec)) = parse_input();

    let to_beat = bfs(&map, sr, sc, er, ec, usize::MAX);

    let mut saved_seconds: HashMap<usize, usize> = HashMap::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if matches!(map[row][col], Wall) {
                // Remove this wall and check the path again
                // but only remove if it has atleast two emtpy spaces next to it

                let mut empty = 0;

                for (dr, dc) in DIRS {
                    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

                    if utils::is_in_bounds(&map, nr, nc)
                        && !matches!(map[nr as usize][nc as usize], Wall)
                    {
                        empty += 1;
                    }
                }

                // empty >= 2 because we need to come from one side and leave from the other
                // remove this wall
                if empty >= 2 {
                    map[row][col] = Empty;

                    let new_record = bfs(&map, sr, sc, er, ec, to_beat);

                    map[row][col] = Wall;

                    if new_record != usize::MAX {
                        let seconds_saved = to_beat - new_record;

                        match saved_seconds.get_mut(&seconds_saved) {
                            Some(v) => *v += 1,

                            None => {
                                saved_seconds.insert(seconds_saved, 1);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut more_than_100 = 0;

    for (k, v) in saved_seconds.iter() {
        if *k >= 100 {
            more_than_100 += v;
        }
    }

    println!("Day20 P1: {more_than_100}");
}

pub fn part2() {
    let (map, (sr, sc), (er, ec)) = parse_input();

    let _to_beat = bfs(&map, sr, sc, er, ec, usize::MAX);

    let mut _saved_seconds: HashMap<usize, usize> = HashMap::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if matches!(map[row][col], Wall) {}
        }
    }

    println!("{_saved_seconds:?}");
}
