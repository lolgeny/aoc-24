#![feature(iter_map_windows)]
use std::{fmt::Display, iter::once};

use aoc24::load_input;
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
        (prices, diffs)
    }).collect_vec();
    // println!("{buyers:?}");
    once(-9..=9).cycle().take(4).multi_cartesian_product().par_bridge().map(|x| (x[0], x[1], x[2], x[3]))
    .map(|(a, b, c, d)| {
        buyers.iter().map(
            |(prices, diffs)| diffs.iter().map_windows(|[i, j, k, l]|
                (**i, **j, **k, **l)
            ).enumerate().filter(|&(_, t)| t == (a, b, c, d))
            .next().map(|(i, _)| prices[i+4]).unwrap_or(0)
        ).sum::<i64>()
    }).max().unwrap()
}

fn main() {
    let inp = load_input("day22");
    println!("{}", part2(&inp));
}