#![feature(iter_map_windows)]
use std::fmt::Display;

use aoc24::load_input;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn part1(inp: &str) -> impl Display {
    inp.lines().map(|l| {
        let mut i = l.parse::<i64>().unwrap();
        let op = [
            |x| x * 64, |x| x / 32, |x| x * 2048
        ];
        for _ in 0..2000 {
            for o in &op {
                let a = o(i);
                i = (i ^ a) % 16777216;
            }
        }
        i
    }).sum::<i64>()
}

fn part2(inp: &str) -> impl Display {
    let buyers = inp.lines().map(|l| {
        let mut i = l.parse::<i64>().unwrap();
        let op = [
            |x| x * 64, |x| x / 32, |x| x * 2048
        ];
        let mut diffs = vec![];
        let mut prices = vec![i % 10];
        for _ in 0..2000 {
            let mut i2 = i;
            for o in &op {
                let a = o(i2);
                i2 = (i2 ^ a) % 16777216;
            }
            diffs.push(i2 % 10 - i % 10);
            prices.push(i2 % 10);
            i = i2;
        }
        let diff_lookup: HashMap<(i64, i64, i64, i64), i64> = diffs.windows(4).enumerate().map(|(i, w)| {
            ((w[0], w[1], w[2], w[3]), prices[i+4])
        }).rev().collect();
        diff_lookup
    }).collect_vec();

    buyers.iter().map(|x| x.keys()).flatten().collect::<HashSet<_>>().into_iter()
    .par_bridge()
    .map(|&(a, b, c, d)| {
        buyers.iter().map(
            |diffs|
                diffs.get(&(a,b,c,d)).copied().unwrap_or(0)
        ).sum::<i64>()
    }).max().unwrap()
}

fn main() {
    let inp = load_input("day22");
    println!("{}", part2(&inp));
}