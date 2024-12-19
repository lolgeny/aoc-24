#![cfg_attr(test, feature(test))]
#![allow(unused)]

#[cfg(test)] extern crate test;

use std::{array, collections::HashMap};

use aoc24::load_input;
use itertools::Itertools;
use mimalloc::MiMalloc;
use smallvec::SmallVec;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

const BUCKET_SIZE: usize = 8;

fn possible(design: &[u8], buckets: &[SmallVec<[&[u8]; BUCKET_SIZE]>; 26]) -> bool {
    let mut possibilities = vec![false; design.len()+1];
    possibilities[design.len()] = true;
    'possibilities: for i in (0..design.len()).rev() {
        for &p in &buckets[(design[i]-'a' as u8) as usize] {
            if i+p.len() > design.len() {continue};
            if &design[i..i+p.len()] == p && possibilities[i+p.len()] {
                possibilities[i] = true;
                continue 'possibilities;
            }
        }
    }
    possibilities[0]
}

fn part1(inp: &str) -> usize {
    let (pats, designs) = inp.split("\n\n").collect_tuple().unwrap();
    let pats = pats.split(", ").map(|x| x.as_bytes()).collect_vec();
    let mut buckets: [SmallVec<[&[u8]; BUCKET_SIZE]>; 26] = array::from_fn(|_| SmallVec::new());
    for p in pats {
        buckets[(p[0]-'a' as u8) as usize].push(p);
    }
    designs.lines()
        .filter(|&d| possible(d.as_bytes(), &buckets))
        .count()
}

fn possible2(design: &[u8], buckets: &[SmallVec<[&[u8]; BUCKET_SIZE]>; 26]) -> u64 {
    let d_len = design.len();
    let mut possibilities = vec![0; d_len+1];
    possibilities[design.len()] = 1;
    for i in (0..d_len).rev() {
        let mut total = 0;
        for &p in &buckets[(design[i]-'a' as u8) as usize] {
            let j = i+p.len();
            if j > d_len {continue};
            if &design[i..j] == p {
                total += possibilities[i+p.len()];
            }
        }
        possibilities[i] = total;
    }
    possibilities[0]
}

fn part2(inp: &str) -> u64 {
    let (pats, designs) = inp.split("\n\n").collect_tuple().unwrap();
    let pats = pats.split(", ").map(|x| x.as_bytes()).collect_vec();
    let mut buckets: [SmallVec<[&[u8]; BUCKET_SIZE]>; 26] = array::from_fn(|_| SmallVec::new());
    for p in pats {
        buckets[(p[0]-'a' as u8) as usize].push(p);
    }
    designs.lines()
        .map(|d| possible2(d.as_bytes(), &buckets))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use aoc24::load_input;
    use test::Bencher;
    use crate::*;

    #[test]
    fn test_parts() {
        let inp = load_input("day19");
        assert_eq!(part1(&inp), 296);
        assert_eq!(part2(&inp), 619970556776002);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let inp = load_input("day19");
        b.iter(|| {
            part1(&inp)
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let inp = load_input("day19");
        b.iter(|| {
            part2(&inp)
        });
    }
}

fn main() {
    let inp = load_input("day19");
    println!("{}", part2(&inp));
}