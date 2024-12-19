#![allow(unused)]

use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fmt::Display};

use aoc24::load_input;
use itertools::Itertools;

const GRID_SIZE: i64 = 70;
const N: usize = 1024;

fn part1(inp: &str) -> impl Display {
    let obstacles_generator: Vec<(i64,i64)> = inp.lines().map(|l| l.split(',').map(|x| x.parse::<i64>().unwrap())
        .collect_tuple().unwrap()
    ).collect_vec();
    let mut i = 0;

    let mut obstacles = HashSet::new();

    'search: loop {
        obstacles.insert(obstacles_generator[i]);
        i += 1;


        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        queue.push((Reverse(0), (0, 0)));
        while let Some((Reverse(score), (x, y))) = queue.pop() {
            if (x, y) == (GRID_SIZE, GRID_SIZE) {
                continue 'search;
            }
            if visited.contains(&(x,y)) {continue};
            visited.insert((x,y));
            for (x2, y2) in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)] {
                if x2 < 0 || y2 < 0 || x2 > GRID_SIZE || y2 > GRID_SIZE {continue};
                if obstacles.contains(&(x2,y2)) {continue};
                queue.push((Reverse(score+1),(x2,y2)));
            }
        }
        return format!("{},{}",obstacles_generator[i-1].0,obstacles_generator[i-1].1);
    }
    // for y in 0..=GRID_SIZE {
    //     for x in 0..=GRID_SIZE {
    //         if obstacles.contains(&(x,y)) {print!("#");}
    //         else if visited.contains(&(x,y)) {print!("O");}
    //         else {print!(".")};
    //     }
    //     println!();
    // }
    // 0
}

fn main() {
    let inp = load_input("day18");
    println!("{}", part1(&inp));
}