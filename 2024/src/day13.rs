use std::u64;

use crate::utils;

#[derive(Default, Clone, Copy, Debug)]
struct Value {
    a: (u64, u64),
    b: (u64, u64),
    x: u64,
    y: u64,
}

type Cache = [[i64; 100]; 100];

fn parse_input() -> Vec<Value> {
    let file_contents = utils::read_file_to_string("./inputs/13");

    let mut values: Vec<Value> = vec![];

    let mut current_value: Value = Value::default();

    for line in file_contents.lines() {
        if line.trim().len() == 0 {
            continue;
        }

        let mut split = line.split(":");

        match split.next() {
            Some(thingy) => match thingy {
                "Prize" => {
                    // X=12748, Y=12176
                    let xy = split.next().unwrap();

                    let mut comma_split = xy.split(",");

                    let x = comma_split.next().unwrap();
                    let y = comma_split.next().unwrap();

                    current_value.x = x.split("=").nth(1).unwrap().parse::<u64>().unwrap();
                    current_value.y = y.split("=").nth(1).unwrap().parse::<u64>().unwrap();

                    values.push(current_value.clone());

                    current_value = Value::default();
                }

                // Button A: X+94, Y+34
                "Button A" => {
                    let xy = split.next().unwrap();
                    let mut comma_split = xy.split(",");

                    let x = comma_split.next().unwrap();
                    let y = comma_split.next().unwrap();

                    current_value.a = (
                        x.split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                        y.split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                    );
                }

                "Button B" => {
                    let xy = split.next().unwrap();
                    let mut comma_split = xy.split(",");
                    let x = comma_split.next().unwrap();
                    let y = comma_split.next().unwrap();

                    current_value.b = (
                        x.split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                        y.split("+").nth(1).unwrap().parse::<u64>().unwrap(),
                    );
                }

                _ => panic!("wtf"),
            },

            None => todo!(),
        };
    }

    return values;
}

fn can_reach_prize(
    value: &mut Value,
    current_x: u64,
    current_y: u64,
    a_press: u64,
    b_press: u64,
    depth: u64,
    cache: &mut Cache,
) -> u64 {
    if current_x > value.x || current_y > value.y || a_press >= 100 || b_press >= 100 {
        return u64::MAX;
    }

    if cache[a_press as usize][b_press as usize] != -1 {
        return cache[a_press as usize][b_press as usize] as u64;
    }

    if value.x == current_x && value.y == current_y {
        return 3 * a_press + b_press;
    }

    let mut cost = u64::MAX;

    // Press button a
    let btn_a = can_reach_prize(
        value,
        current_x + value.a.0,
        current_y + value.a.1,
        a_press + 1,
        b_press,
        depth + 1,
        cache,
    );

    // Press button b
    let btn_b = can_reach_prize(
        value,
        current_x + value.b.0,
        current_y + value.b.1,
        a_press,
        b_press + 1,
        depth + 1,
        cache,
    );

    if btn_a == u64::MAX && btn_b == u64::MAX {
        cache[a_press as usize][b_press as usize] = u64::MAX as i64;
        return cost;
    }

    cost = cost.min(btn_a).min(btn_b);

    cache[a_press as usize][b_press as usize] = cost as i64;

    return cost;
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
//
// 94a + 22b = X
// 34a + 67b = Y
//
// (94a + 22b) - (34a + 67b) = X - Y
// 60a - 45b = 3000
//
// a = (3000 + 45(Y - 34a) / 67) / 60
fn can_reach_prize_2() -> u64 {
}

pub fn day13p1() {
    let mut values = parse_input();

    println!("{values:?}");

    let mut cache = [[-1; 100]; 100];

    let mut total = 0;

    for v in &mut values {
        let res = can_reach_prize(v, 0, 0, 0, 0, 0, &mut cache);
        cache = [[-1; 100]; 100];

        println!("v: {v:?}, res: {res}");

        if res != u64::MAX {
            total += res;
        }
    }

    println!("Day13 P1: {total}");
}

pub fn day13p2() {
    let mut values = parse_input();

    // println!("{values:?}");

    // let mut cache = [[-1; 100]; 100];

    let mut total = 0;

    for v in &mut values[1..2] {
        v.x += 10000000000000;
        v.y += 10000000000000;

        let res = can_reach_prize_2(v, v.x, v.y, 0, 0);
        // cache = [[-1; 100]; 100];

        println!("v: {v:?}, res: {res}");

        if res != u64::MAX {
            total += res;
        }
    }

    println!("Day13 P1: {total}");
}
