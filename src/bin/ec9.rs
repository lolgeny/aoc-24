#![allow(unused)]

use std::{fmt::Display, usize};

use aoc24::load_input;

const STAMPS: &[usize] = &[1, 3, 5, 10];
const STAMPS2: &[usize] = &[1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
const STAMPS3: &[usize] = &[1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101];

fn create_all(n: usize, stamps: &[usize]) -> Vec<usize> {
    let mut creations = vec![0; n + 1];
    creations[0] = 0;
    for i in 1..=n {
        creations[i] = stamps.iter().filter(|s| **s <= i).map(|s| creations[i-*s]).min().unwrap() + 1;
    }
    creations
}

fn create(n: usize, stamps: &[usize]) -> usize {
    create_all(n, stamps)[n]
}

fn part1(inp: &str) -> impl Display {
    inp.lines().map(|x| {
        let k = x.parse::<usize>().unwrap();
        create(k, STAMPS)
    }).sum::<usize>()
}

fn part2(inp: &str) -> impl Display {
    inp.lines().map(|x| {
        let k = x.parse::<usize>().unwrap();
        create(k, STAMPS2)
    }).sum::<usize>()
}

fn part3(inp: &str) -> impl Display {
    inp.lines().map(|x| {
        let k = x.parse::<usize>().unwrap();
        let mut i = k/2 - 52;
        let costs = create_all(k, STAMPS3);
        let mut best = usize::MAX;
        let mut best_i = 0;
        while (k-i).abs_diff(i) > 100 {i += 1};
        while (k - i).abs_diff(i) <= 100 {
            let j = k - i;
            let cost = costs[i] + costs[j];
            if cost < best {
                best = cost; best_i = i;
            }
            i += 1;
        }
        best
    }).sum::<usize>()
}

fn main() {
    let inp = load_input("ec9");
    println!("{}", part3(&inp));
}