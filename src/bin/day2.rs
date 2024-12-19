#![allow(unused)]
use aoc24::load_input;
use itertools::Itertools;

fn safe(ns: &[i64]) -> bool {
    let dif: Vec<i64> = (0..ns.len()-1).map(|i| ns[i+1]-ns[i]).collect_vec();
    dif.iter().all(|x| 1 <= x.abs() && x.abs() <= 3)
        && (dif.iter().all(|x| *x > 0) || dif.iter().all(|x| *x < 0))
}

fn part1(inp: &str) -> u64 {
    inp.lines().filter(|l| {
        let mut ns = l.split(' ').map(|x| x.parse::<i64>().unwrap()).collect_vec();
        if safe(&ns) {return true};
        // for i in 0..ns.len() {
            // let x = ns.remove(i);
            // if safe(&ns) {return true};
            // ns.insert(i, x);
        // }
        false
    }).count() as u64
}

fn main() {
    let inp = load_input("day2");
    println!("{}", part1(&inp));
}