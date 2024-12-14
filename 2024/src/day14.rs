use std::{fs::File, io::BufWriter, usize};

use png::{self, ColorType, Encoder};

use crate::utils;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

type Vel = Pos;

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn move_robot(&mut self, rows: usize, cols: usize) {
        let final_x = self.pos.x + self.vel.x;

        if final_x < 0 {
            // wrap around by that many thingies
            self.pos.x = (final_x % cols as i32 + (cols as i32)) % cols as i32;
        } else {
            self.pos.x = final_x % cols as i32;
        }

        let final_y = self.pos.y + self.vel.y;

        if final_y < 0 {
            // wrap around by that many thingies
            self.pos.y = (final_y % rows as i32 + (rows as i32)) % rows as i32;
        } else {
            self.pos.y = final_y % rows as i32;
        }

        assert!(self.pos.x >= 0 && self.pos.x < cols as i32);
        assert!(self.pos.y >= 0 && self.pos.y < rows as i32);
    }
}

fn parse_input() -> Vec<Robot> {
    let file_contents = utils::read_file_to_string("./inputs/14");

    let mut robots: Vec<Robot> = vec![];

    for line in file_contents.lines() {
        let mut split = line.split_whitespace();

        let mut pos = split.next().unwrap();
        let mut vel = split.next().unwrap();

        pos = pos.split("=").nth(1).unwrap();
        vel = vel.split("=").nth(1).unwrap();

        robots.push(Robot {
            pos: Pos {
                x: pos.split(",").nth(0).unwrap().parse::<i32>().unwrap(),
                y: pos.split(",").nth(1).unwrap().parse::<i32>().unwrap(),
            },

            vel: Vel {
                x: vel.split(",").nth(0).unwrap().parse::<i32>().unwrap(),
                y: vel.split(",").nth(1).unwrap().parse::<i32>().unwrap(),
            },
        });
    }

    return robots;
}

fn print_positions(row: usize, col: usize, robots: &Vec<Robot>) {
    let mut v = vec![vec![0; col]; row];

    for r in robots {
        v[r.pos.y as usize][r.pos.x as usize] += 1;
    }

    for row in v {
        for c in row {
            if c > 0 {
                print!("{c}")
            } else {
                print!(".")
            };
        }

        println!();
    }
}

// WE WILL PRINT ALL THE PNGSSS
fn print_png(rows: usize, cols: usize, robots: &Vec<Robot>, index: usize) {
    let mut v = vec![vec![0; cols]; rows];

    for r in robots {
        v[r.pos.y as usize][r.pos.x as usize] += 1;
    }

    let mut pngvec = vec![0; rows * cols];

    for row in 0..v.len() {
        for col in 0..v[0].len() {
            if v[row][col] > 0 {
                let idx = row * cols + col;
                pngvec[idx] = 255;
            }
        }
    }

    let file = File::create(format!("{index}.png")).unwrap();
    let writer = BufWriter::new(file);

    let mut encoder = Encoder::new(writer, cols as u32, rows as u32);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pngvec).unwrap();
}

pub fn day14p1() {
    let mut robots = parse_input();

    let grid_rows = 103;
    let grid_cols = 101;

    let iterations = 100;

    for _ in 0..iterations {
        for r in &mut robots {
            r.move_robot(grid_rows, grid_cols);
        }

        print_positions(grid_rows, grid_cols, &robots);
    }

    let (mut quad1, mut quad2, mut quad3, mut quad4) = (0, 0, 0, 0);

    for r in &robots {
        // figure out what quadrant they're in and multiply those together

        if (r.pos.x as usize) < grid_cols / 2 {
            // in the 1st or 3rd quad
            if (r.pos.y as usize) < grid_rows / 2 {
                quad1 += 1;
            } else if (r.pos.y as usize) > grid_rows / 2 {
                quad3 += 1;
            }
        } else if (r.pos.x as usize) > grid_cols / 2 {
            // in the 2nd or 4th
            if (r.pos.y as usize) < grid_rows / 2 {
                quad2 += 1;
            } else if (r.pos.y as usize) > grid_rows / 2 {
                quad4 += 1;
            }
        }
    }

    // print_positions(grid_rows, grid_cols, &robots);

    println!("Quad1: {quad1}, Quad2: {quad2}, Quad3: {quad3}, Quad4: {quad4}");

    println!("Day14 P1: {}", quad1 * quad2 * quad3 * quad4);
}

pub fn day14p2() {
    let mut robots = parse_input();

    let grid_rows = 103;
    let grid_cols = 101;

    let iterations = 7000;

    for i in 0..iterations {
        for r in &mut robots {
            r.move_robot(grid_rows, grid_cols);
        }

        print_png(grid_rows, grid_cols, &robots, i);
    }
}
