#![cfg_attr(test, feature(test))]
#![allow(unused)]

#[cfg(test)] extern crate test;

use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use aoc24::load_input;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use partitions::{partition_vec, PartitionVec};

const GRID_SIZE: i64 = 70;
const N: usize = 1024;

fn part1(inp: &str) -> impl Display {
    let obstacles: Vec<(i64,i64)> = inp.lines().map(|l| l.split(',').map(|x| x.parse::<i64>().unwrap())
        .collect_tuple().unwrap()
    ).take(N).collect_vec();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push((Reverse(0), (0, 0)));
    while let Some((Reverse(score), (x, y))) = queue.pop() {
        if (x, y) == (GRID_SIZE, GRID_SIZE) {
            return score;
        }
        if visited.contains(&(x,y)) {continue};
        visited.insert((x,y));
        for (x2, y2) in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)] {
            if x2 < 0 || y2 < 0 || x2 > GRID_SIZE || y2 > GRID_SIZE {continue};
            if obstacles.contains(&(x2,y2)) {continue};
            queue.push((Reverse(score+1),(x2,y2)));
        }
    }
    panic!("Problem not solvable")
}

fn part2(inp: &str) -> impl Display {
    let obstacles_generator: Vec<(i64,i64)> = inp.lines().map(|l| l.split(',').map(|x| x.parse::<i64>().unwrap())
        .collect_tuple().unwrap()
    ).collect_vec();
    let mut i = 0;

    let mut obstacles = PartitionVec::with_capacity(obstacles_generator.len()+2);
    obstacles.push((-1,0));
    obstacles.push((0,-1));
    let mut obst_indices = HashMap::new();

    for obstacle in obstacles_generator {
        obstacles.push(obstacle);
        let i = obstacles.len()-1;
        for (dx, dy) in [(1,0),(1,1),(1,-1),(0,1),(0,-1),(-1,-1),(-1,0),(-1,1)] {
            if let Some(&j) = obst_indices.get(&(obstacle.0+dx, obstacle.1+dy)) {
                obstacles.union(i, j);
            }
        }
        if obstacle.0 == 0 || obstacle.1 == GRID_SIZE {obstacles.union(0, i);}
        if obstacle.0 == GRID_SIZE || obstacle.1 == 0 {obstacles.union(1, i);}
        if obstacles.same_set(0,1) {return format!("{},{}", obstacle.0, obstacle.1)};
        obst_indices.insert(obstacle, i);
    }
    panic!("Problem not solvable")
}

#[cfg(test)]
mod tests {
    use aoc24::load_input;
    use test::Bencher;
    use crate::*;

    #[test]
    fn text_day18() {
        let inp = load_input("day18");
        assert_eq!(format!("{}", part1(&inp)), "320");
        assert_eq!(format!("{}", part2(&inp)), "34,40");
    }

    #[bench]
    fn bench_day18_part1(b: &mut Bencher) {
        let inp = load_input("day18");
        b.iter(|| part1(&inp));
    }

    #[bench]
    fn bench_day18_part2(b: &mut Bencher) {
        let inp = load_input("day18");
        b.iter(|| part2(&inp));
    }
}

fn main() {
    let inp = load_input("day18");
    println!("{}", part2(&inp));
}