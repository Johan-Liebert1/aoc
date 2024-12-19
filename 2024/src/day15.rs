use core::panic;
use std::{fmt::Display, i32, usize};

use crate::utils;

#[derive(Debug, Copy, Clone)]
enum MapP1 {
    Wall,
    Robot,
    Box,
    Empty,
}

impl Display for MapP1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            MapP1::Wall => "#",
            MapP1::Robot => "@",
            MapP1::Box => "O",
            MapP1::Empty => ".",
        };

        write!(f, "{}", v)
    }
}

#[derive(Debug, Copy, Clone)]
enum MapP2 {
    Wall,
    Robot,
    BoxLeft,
    BoxRight,
    Empty,
}

impl Display for MapP2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            MapP2::Wall => "#",
            MapP2::Robot => "@",
            MapP2::BoxLeft => "[",
            MapP2::BoxRight => "]",
            MapP2::Empty => ".",
        };

        write!(f, "{}", v)
    }
}

#[derive(Debug)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
}

fn is_in_bounds<T: Sized>(map: &Vec<Vec<T>>, row: i32, col: i32) -> bool {
    return row >= 0 && col >= 0 && row < map.len() as i32 && col < map[0].len() as i32;
}

fn make_move_p1(
    map: &mut Vec<Vec<MapP1>>,
    robot_move: &Moves,
    initial_robot_r: i32,
    initial_robot_c: i32,
) -> (usize, usize) {
    use MapP1::*;

    let r = initial_robot_r;
    let c = initial_robot_c;

    let (ra, ca) = match robot_move {
        Moves::Up => (-1, 0),
        Moves::Down => (1, 0),
        Moves::Left => (0, -1),
        Moves::Right => (0, 1),
    };

    if !is_in_bounds(map, initial_robot_r + ra, initial_robot_c + ca) {
        return (r as usize, c as usize);
    }

    match map[(r + ra) as usize][(c + ca) as usize] {
        Wall => return (r as usize, c as usize),

        Robot => {
            panic!("why is there another robot at ({},{}). Initial robot pos ({initial_robot_r},{initial_robot_c})", r + ra, c + ca)
        }

        Box => {
            // find the first empty space
            let (mut erow, mut ecol) = (initial_robot_r + ra * 2, initial_robot_c + ca * 2);

            while is_in_bounds(map, erow, ecol) {
                match map[erow as usize][ecol as usize] {
                    Wall | Empty => break,

                    Box => {
                        erow += ra;
                        ecol += ca;
                    }

                    Robot => panic!("Why another robot???"),
                }
            }

            // this one has to be empty, only then can we move
            if !matches!(map[erow as usize][ecol as usize], Empty) {
                return (r as usize, c as usize);
            }

            // (erow, ecol) is empty
            // move all the boxes here
            while !(erow == initial_robot_r && ecol == initial_robot_c) {
                (
                    map[erow as usize][ecol as usize],
                    map[(erow - ra) as usize][(ecol - ca) as usize],
                ) = (
                    map[(erow - ra) as usize][(ecol - ca) as usize],
                    map[erow as usize][ecol as usize],
                );

                erow -= ra;
                ecol -= ca;
            }

            let final_row = (r + ra) as usize;
            let final_col = (c + ca) as usize;

            return (final_row, final_col);
        }

        Empty => {
            // empty space up, simply move the robot

            let final_row = (r + ra) as usize;
            let final_col = (c + ca) as usize;

            (
                map[final_row][final_col],
                map[initial_robot_r as usize][initial_robot_c as usize],
            ) = (
                map[initial_robot_r as usize][initial_robot_c as usize],
                map[final_row][final_col],
            );

            return (final_row, final_col);
        }
    }
}

fn parse_input_p1() -> (Vec<Vec<MapP1>>, Vec<Moves>, (usize, usize)) {
    let file_contents = utils::read_file_to_string("./inputs/15");

    let mut split = file_contents.split("\n\n");

    let mut vec: Vec<Vec<MapP1>> = vec![];
    let mut moves: Vec<Moves> = vec![];

    let (mut robot_row, mut robot_col) = (0, 0);

    for (row, line) in split.next().unwrap().lines().enumerate() {
        if line.trim().len() == 0 {
            continue;
        }

        let v: Vec<MapP1> = line
            .chars()
            .enumerate()
            .map(|(col, x)| match x {
                '#' => MapP1::Wall,
                '@' => {
                    robot_row = row;
                    robot_col = col;

                    MapP1::Robot
                }
                'O' => MapP1::Box,
                '.' => MapP1::Empty,

                _ => panic!("wtf"),
            })
            .collect();

        vec.push(v);
    }

    for line in split.next().unwrap().lines() {
        if line.trim().len() == 0 {
            continue;
        }

        moves.extend(line.chars().map(|x| match x {
            '^' => Moves::Up,
            '<' => Moves::Left,
            '>' => Moves::Right,
            'v' => Moves::Down,

            _ => panic!("wtf"),
        }));
    }

    return (vec, moves, (robot_row, robot_col));
}

fn parse_input_p2() -> (Vec<Vec<MapP2>>, Vec<Moves>, (usize, usize)) {
    let file_contents = utils::read_file_to_string("./inputs/15");

    let mut split = file_contents.split("\n\n");

    let mut vec: Vec<Vec<MapP2>> = vec![];
    let mut moves: Vec<Moves> = vec![];

    let (mut robot_row, mut robot_col) = (0, 0);

    for (row, line) in split.next().unwrap().lines().enumerate() {
        if line.trim().len() == 0 {
            continue;
        }

        let v: Vec<MapP2> = line
            .chars()
            .enumerate()
            .map(|(col, x)| match x {
                '#' => [MapP2::Wall, MapP2::Wall].to_vec(),
                '@' => {
                    robot_row = row;
                    robot_col = col * 2;
                    [MapP2::Robot, MapP2::Empty].to_vec()
                }
                'O' => [MapP2::BoxLeft, MapP2::BoxRight].to_vec(),
                '.' => [MapP2::Empty, MapP2::Empty].to_vec(),

                _ => panic!("wtf"),
            })
            .flatten()
            .collect();

        vec.push(v);
    }

    for line in split.next().unwrap().lines() {
        if line.trim().len() == 0 {
            continue;
        }

        moves.extend(line.chars().map(|x| match x {
            '^' => Moves::Up,
            '<' => Moves::Left,
            '>' => Moves::Right,
            'v' => Moves::Down,

            _ => panic!("wtf"),
        }));
    }

    return (vec, moves, (robot_row, robot_col));
}

fn make_move_p2(
    map: &mut Vec<Vec<MapP2>>,
    robot_move: &Moves,
    initial_robot_r: i32,
    initial_robot_c: i32,
) -> (usize, usize) {
    use MapP2::*;

    let r = initial_robot_r;
    let c = initial_robot_c;

    match robot_move {
        Moves::Up | Moves::Down => {
            let ra = if matches!(robot_move, Moves::Up) {
                -1
            } else {
                1
            };

            let mut to_move: Vec<(i32, i32)> = vec![(initial_robot_r, initial_robot_c)];
            let mut queue: Vec<(i32, i32)> = vec![(initial_robot_r + ra, initial_robot_c)];

            while queue.len() > 0 {
                let (erow, ecol) = queue[0];
                queue = queue[1..].to_vec();

                if !is_in_bounds(map, erow, ecol) {
                    continue;
                }

                match map[(erow) as usize][ecol as usize] {
                    Empty => continue,

                    // Can't move one box, so can't move the entire set
                    Wall => return (initial_robot_r as usize, initial_robot_c as usize),

                    // we have '[' above of us
                    BoxLeft => {
                        if !to_move.contains(&(erow, ecol)) {
                            to_move.push((erow, ecol));
                            queue.push((erow + ra, ecol));
                        }

                        if !to_move.contains(&(erow, ecol + 1)) {
                            to_move.push((erow, ecol + 1));
                            queue.push((erow + ra, ecol + 1));
                        }
                    }

                    // we have ']' above of us
                    BoxRight => {
                        if !to_move.contains(&(erow, ecol)) {
                            to_move.push((erow, ecol));
                            queue.push((erow + ra, ecol));
                        }

                        if !to_move.contains(&(erow, ecol - 1)) {
                            to_move.push((erow, ecol - 1));
                            queue.push((erow + ra, ecol - 1));
                        }
                    }

                    Robot => {
                        println!(
                            "================({},{ecol}) Move: ({robot_move:?})=================",
                            erow + ra
                        );
                        print_map(map);
                        panic!("bruh found robot bruh");
                    }
                }
            }

            // move all to_moves by ra
            for (r, c) in to_move.iter().rev() {
                (
                    map[*r as usize][*c as usize],
                    map[(r + ra) as usize][*c as usize],
                ) = (
                    map[(r + ra) as usize][*c as usize],
                    map[*r as usize][*c as usize],
                );
            }

            return ((initial_robot_r + ra) as usize, initial_robot_c as usize);
        }

        Moves::Left | Moves::Right => {
            let ca = if matches!(robot_move, Moves::Left) {
                -1
            } else {
                1
            };

            let (erow, mut ecol) = (initial_robot_r, initial_robot_c + ca);

            // find the first space or wall
            while is_in_bounds(map, erow, ecol) {
                match map[erow as usize][ecol as usize] {
                    Wall | Empty => break,

                    BoxLeft | BoxRight => {
                        ecol += ca;
                    }

                    Robot => {
                        print_map(map);
                        panic!("Why another robot???")
                    }
                }
            }

            // this one has to be empty, only then can we move
            if !matches!(map[erow as usize][ecol as usize], Empty) {
                return (r as usize, c as usize);
            }

            while !(erow == initial_robot_r && ecol == initial_robot_c) {
                (
                    map[erow as usize][ecol as usize],
                    map[(erow) as usize][(ecol - ca) as usize],
                ) = (
                    map[(erow) as usize][(ecol - ca) as usize],
                    map[erow as usize][ecol as usize],
                );

                ecol -= ca;
            }

            let final_row = r as usize;
            let final_col = (c + ca) as usize;

            return (final_row, final_col);
        }
    };
}

fn print_map<T: Display>(map: &Vec<Vec<T>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }

        println!("");
    }
}

pub fn part1() {
    let (mut map, moves, (mut robot_row, mut robot_col)) = parse_input_p1();

    for robot_move in &moves {
        (robot_row, robot_col) =
            make_move_p1(&mut map, robot_move, robot_row as i32, robot_col as i32);
    }

    let mut total = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if matches!(map[row][col], MapP1::Box) {
                total += 100 * row + col;
            }
        }
    }

    println!("Day15 P1: {total}");
}

pub fn part2() {
    let (mut map, moves, (mut robot_row, mut robot_col)) = parse_input_p2();

    for robot_move in &moves {
        (robot_row, robot_col) =
            make_move_p2(&mut map, robot_move, robot_row as i32, robot_col as i32);
    }

    let mut total = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if matches!(map[row][col], MapP2::BoxLeft) {
                total += 100 * row + col;
            }
        }
    }

    print_map(&map);

    println!("Day15 P2: {total}");
}
