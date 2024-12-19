#![cfg_attr(test, feature(test))]
#![allow(unused)]

#[cfg(test)] extern crate test;


use aoc24::load_input;
use itertools::Itertools;

fn possible(design: &str, pats: &[&str]) -> bool {
    let mut possibilities = vec![false; design.len()+1];
    possibilities[design.len()] = true;
    'possibilities: for i in (0..design.len()).rev() {
        for &p in pats {
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
    let pats = pats.split(", ").map(|x: &str| x).collect_vec();
    designs.lines()
        .filter(|&d| possible(d, &pats))
        .count()
}

fn possible2(design: &str, pats: &[&str]) -> u64 {
    let mut possibilities = vec![0; design.len()+1];
    possibilities[design.len()] = 1;
    for i in (0..design.len()).rev() {
        let mut total = 0;
        for &p in pats {
            if i+p.len() > design.len() {continue};
            if &design[i..i+p.len()] == p {
                total += possibilities[i+p.len()];
            }
        }
        possibilities[i] = total;
    }
    possibilities[0]
}

fn part2(inp: &str) -> u64 {
    let (pats, designs) = inp.split("\n\n").collect_tuple().unwrap();
    let pats = pats.split(", ").map(|x| x).collect_vec();
    designs.lines()
        .map(|ref d| possible2(d, &pats))
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
    println!("{}", part1(&inp));
}