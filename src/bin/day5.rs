use std::cmp::Ordering;

use aoc24::load_input;
use itertools::Itertools;

fn fix(mut o: Vec<usize>, rules: &[(usize, usize)]) -> Vec<usize> {
    o.sort_by(|&a, &b| {
        rules.iter().filter(|r| **r == (a,b) || **r == (b,a)).next()
            .map(|&r| if r == (a,b) {Ordering::Less} else {Ordering::Greater})
            .unwrap_or(Ordering::Equal)
    });
    o
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Part {Part1, Part2}
fn solve(inp: &str, part: Part) -> usize {
    let (rules, orders) = inp.split("\n\n").collect_tuple().unwrap();
    let rules: Vec<(usize, usize)> = rules.split('\n')
        .map(|l| l.split('|').map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap())
        .collect_vec();
    let orders = orders.split('\n')
        .map(|l| l.split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec()).collect_vec();

    orders.into_iter().map(|o| {
        let mid = (o.len() - 1)/2;
        if (0..o.len()).any(|i| (0..i).any(|j| {
            rules.iter().any(|(x, y)| *x == o[i] && *y == o[j])
        })) {if part == Part::Part1 {0} else {fix(o, &rules)[mid]}} else {if part == Part::Part1 {o[mid]} else {0}}
    }).sum()
}

fn main() {
    let inp = load_input("day5");
    println!("{}", solve(&inp, Part::Part2));
}