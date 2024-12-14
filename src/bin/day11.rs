#![feature(let_chains)]

use std::collections::HashMap;

use aoc24::load_input;
use itertools::Itertools;

fn try_split(n: u64) -> Option<(u64, u64)> {
    let d = ((n as f64).log10().floor()) as u32 + 1;
    if d % 2 == 1 {return None};
    let f = 10u64.pow(d/2);
    let b = n % f;
    Some(((n-b)/f, b))
}

fn part1(inp: &str) -> usize {
    let mut stones: HashMap<u64, usize> = inp.split_ascii_whitespace().map(|x| (x.parse::<u64>().unwrap(), 1)).collect();
    for x in 0..5000 {
        // println!("{x} -- {}", stones.values().sum::<usize>());
        let mut new_stones = HashMap::new();
        for &k in stones.keys() {
            if k == 0 {
                let n = *stones.get(&0).unwrap();
                match new_stones.get_mut(&1) {
                    Some(x) => {*x += n}
                    None => {new_stones.insert(1, n);}
                }
            } else if let Some((a, b)) = try_split(k) {
                let n = *stones.get(&k).unwrap();
                match new_stones.get_mut(&a) {
                    Some(x) => {*x += n}
                    None => {new_stones.insert(a, n);}
                }
                match new_stones.get_mut(&b) {
                    Some(x) => {*x += n}
                    None => {new_stones.insert(b, n);}
                }
            } else {
                let n = *stones.get(&k).unwrap();
                match new_stones.get_mut(&(k*2024)) {
                    Some(x) => {*x += n}
                    None => {new_stones.insert(k*2024, n);}
                }
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn main() {
    let inp = load_input("day11");
    println!("{}", part1(&inp));
}