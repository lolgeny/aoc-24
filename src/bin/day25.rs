use std::fmt::Display;

use aoc24::load_input;
use itertools::iproduct;

fn part1(inp: &str) -> impl Display {
    let mut locks = vec![];
    let mut keys = vec![];
    for x in inp.split("\n\n") {
        let mut heights = [0, 0, 0, 0, 0];
        let mut is_key = true;
        for (r, l) in x.lines().enumerate() {
            if r == 0 && l == "#####" {is_key = false};
            for (i, c) in l.chars().enumerate() {
                if c == '#' { heights[i] += 1 };
            }
        }
        if is_key { keys.push(heights); }
        else      { locks.push(heights); }
    }
    iproduct!(locks, keys).filter(|(l, k)|
        (0..5).all(|i| l[i] + k[i] <= 7)
    ).count()
}

fn main() {
    let inp = load_input("day25");
    println!("{}", part1(&inp));
}