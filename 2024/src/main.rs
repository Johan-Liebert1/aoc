use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("./inputs/1");

    if !file.is_ok() {
        panic!("yoyo wtf");
    }

    let mut file = file.unwrap();
    let mut file_contents = String::new();

    let _ = file.read_to_string(&mut file_contents);

    let mut v1 = vec![];
    let mut v2 = vec![];

    let mut map: HashMap<i32, i32> = HashMap::new();

    file_contents.split("\n").for_each(|x| {
        if x.len() == 0 {
            return;
        }

        let mut split = x.split(" ");

        let first = split.next().unwrap().parse::<i32>().unwrap();
        let second = split.next().unwrap().parse::<i32>().unwrap();

        v1.push(first);
        v2.push(second);

        match map.get_mut(&second) {
            Some(v) => {
                *v += 1
            },

            None => {
                map.insert(second, 1);
            },
        }
    });

    v1.sort();
    v2.sort();

    let mut total_part_1 = 0;

    for (a, b) in v1.iter().zip(v2) {
        total_part_1 += a.abs_diff(b);
    }

    let mut total_part_2 = 0;

    for a in v1 {
        let to_mul =  match map.get(&a) {
            Some(v) => *v,
            None => 0,
        };

        total_part_2 += a * to_mul;
    }

    println!("Part1: {}", total_part_1);
    println!("Part2: {}", total_part_2);
}
