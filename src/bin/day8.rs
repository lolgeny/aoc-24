use std::collections::HashSet;

use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let freqs: HashSet<char> = grid.iter().flatten().copied().filter(|x| *x != '.').collect();
    let mut antinodes = HashSet::new();
    for f in freqs {
        for x in (0..grid.len() as isize).cartesian_product(0..grid[0].len() as isize)
            .filter(|(i, j)| grid[*i as usize][*j as usize] == f)
            .combinations(2) {
                let (ai, aj) = x[0];
                let (bi, bj) = x[1];
                let di = bi-ai;
                let dj = bj-aj;
                let mut i = ai;
                let mut j = aj;
                while i >= 0 && (i as usize) < grid.len() && j >= 0 && (j as usize) < grid.len() {
                    i -= di; j -= dj;
                }
                i += di; j += dj;
                while i >= 0 && (i as usize) < grid.len() && j >= 0 && (j as usize) < grid.len() {
                    antinodes.insert((i, j));
                    i += di; j += dj;
                }
            }
    }
    antinodes.into_iter()
        .filter(|(i, j)| 0 <= *i && (*i as usize) < grid.len()
            && 0 <= *j && (*j as usize) < grid[0].len())
        .count()
}

fn main() {
    let inp = load_input("day8");
    println!("{}", part1(&inp));
}