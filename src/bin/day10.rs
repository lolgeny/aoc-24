#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let elevations: HashMap<(usize, usize), u8> = (0..x_len).cartesian_product(0..y_len)
        .map(|(x, y)| ((x, y), grid[y][x] as u8 - '0' as u8))
        .collect();
    let mut total = 0;
    for (&(x0, y0), _) in elevations.iter().filter(|(_, v)| **v == 0) {
        let mut queue = vec![(x0, y0)];
        let mut ends = HashSet::new();
        while let Some(n) = queue.pop() {
            let k = *elevations.get(&n).unwrap();
            if k == 9 {
                total += 1;
                ends.insert(n); continue;
            }
            for m in [(n.0.wrapping_sub(1), n.1), (n.0.wrapping_add(1), n.1),
                (n.0, n.1.wrapping_sub(1)), (n.0, n.1.wrapping_add(1))] {
                    if let Some(&l) = elevations.get(&m) && k + 1 == l {
                        queue.push(m);
                    }
                }
        }
        // total += ends.drain().count();
    }
    total
}

fn main() {
    let inp = load_input("day10");
    println!("{}", part1(&inp));
}