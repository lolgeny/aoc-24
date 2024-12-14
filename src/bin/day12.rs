use std::collections::{HashMap, HashSet};

use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let grid: HashMap<(isize, isize), char> = inp.lines().enumerate()
        .map(|(i, l)| l.chars().enumerate()
            .map(move |(j, c)| ((i as isize, j as isize), c)))
        .flatten().collect();

    let mut visited = HashSet::new();
    let mut queue = vec![(0,0)];
    let mut neighbours = vec![];
    let mut a = 0usize;
    let mut p = 0;
    let mut total = 0;
    while let Some(x) = queue.pop() {
        if visited.contains(&x) {continue}
        let g = *grid.get(&x).unwrap();
        neighbours.push(x);
        while let Some(n @ (i, j)) = neighbours.pop() {
            if visited.contains(&n) {continue};
            match grid.get(&n) {
                Some(&l) if l == g => {},
                Some(_) => {queue.push(n); continue}
                None => continue
            };
            visited.insert(n);
            a += 1;
            // 845
            // 1 3
            // 726
            let (e1, e2, e3, e4, e5, e6, e7, e8) =
                [(i-1,j),(i,j-1),(i+1,j),(i,j+1), (i+1,j+1),(i+1,j-1),(i-1,j-1),(i-1,j+1)]
                .into_iter().map(|(i2, j2)| grid.get(&(i2,j2))
                .map_or(true, |x| *x != g))
                .collect_tuple().unwrap();
            p += [(e1, e2, e7), (e2, e3, e6), (e3, e4, e5), (e4, e1, e8)].into_iter()
                .filter(|&(a, b, c)| (a && b) || (!a && !b && c))
                .count();
            neighbours.push((i+1,j));
            neighbours.push((i-1,j));
            neighbours.push((i,j+1));
            neighbours.push((i,j-1));
        }
        total += a*p;
        a = 0;
        p = 0;
    }

    total
}

fn main() {
    let inp = load_input("day12");
    println!("{}", part1(&inp));
}