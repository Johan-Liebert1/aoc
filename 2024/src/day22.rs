use std::{collections::HashMap, vec};

use crate::utils;

pub fn part1() {
    let mut numbers = utils::read_file_to_string("./inputs/22")
        .lines()
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let modd = 16777216;

    for (_, n) in numbers.iter_mut().enumerate() {
        let mut number = *n;

        for _ in 0..2000 {
            let m = number * 64;
            number ^= m;
            number = number % modd;

            let m = number / 32;
            number ^= m;
            number = number % modd;

            let m = number * 2048;
            number ^= m;
            number = number % modd;
        }

        *n = number;
    }

    let total = numbers.iter().sum::<u64>();

    println!("Day22 P1: {total}");
}

// (diff1, diff2, diff3, diff4)
type Cache = HashMap<(i32, i32, i32, i32), usize>;

fn find_window(
    prices_diff: &Vec<Vec<(u64, i32)>>,
    window: &[(u64, i32)],
    prices_diff_idx: usize,
    current_diff_idx: usize,
    cache: &mut Cache,
) -> usize {
    println!("prices_diff_idx: {prices_diff_idx}, current_diff_idx: {current_diff_idx}");

    if prices_diff_idx >= prices_diff.len() {
        return 0;
    }

    if current_diff_idx >= prices_diff[prices_diff_idx].len() {
        return 0;
    }

    let cache_key = (window[0].1, window[1].1, window[2].1, window[3].1);

    if let Some(thingy) = cache.get(&cache_key) {
        return *thingy;
    }

    let mut max_profit = 0;

    for w in prices_diff[prices_diff_idx][current_diff_idx..].windows(4) {
        if w == window {
            max_profit = max_profit.max(find_window(
                prices_diff,
                window,
                prices_diff_idx,
                current_diff_idx + 4,
                cache,
            ));
        }
    }

    cache.insert(cache_key, max_profit);

    return max_profit;
}

pub fn part2() {
    let mut numbers = utils::read_file_to_string("./inputs/22")
        .lines()
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let modd = 16777216;

    // (one's digit, difference)
    let mut prices_diff: Vec<Vec<(u64, i32)>> = vec![];

    for (_, n) in numbers.iter_mut().enumerate() {
        let mut number = *n;

        let mut current_buyer_price_diff: Vec<(u64, i32)> = vec![];

        current_buyer_price_diff.push((number, 0));

        for _ in 0..2000 {
            let m = number * 64;
            number ^= m;
            number = number % modd;

            let m = number / 32;
            number ^= m;
            number = number % modd;

            let m = number * 2048;
            number ^= m;
            number = number % modd;

            current_buyer_price_diff.push((
                number % 10,
                current_buyer_price_diff.last().unwrap().1 - (number % 10) as i32,
            ));
        }

        prices_diff.push(current_buyer_price_diff);
    }

    let mut total = 0;
    let mut cache: Cache = Cache::new();

    for (i, diff) in prices_diff.iter().enumerate() {
        for w in diff.windows(4) {
            total += find_window(&prices_diff, w, i, 0, &mut cache);
        }
    }

    println!("Day22 P2: {total}");
}
