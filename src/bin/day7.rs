#![cfg_attr(test, feature(test))]
#[cfg(test)] extern crate test;

use aoc24::load_input;
use itertools::Itertools;

fn test(target: u64, stack: &[u64]) -> bool {
    if stack.len() == 0 {return false};
    if stack.len() == 1 {return stack[0] == target};
    let i = stack.len()-1;
    let d = 10u64.pow((stack[i] as f64).log10().floor() as u32+ 1);
    (target >= stack[i] && test(target-stack[i], &stack[..i]))
    || (target % stack[i] == 0 && test(target/stack[i], &stack[..i]))
    // || (target % d == stack[i] && test((target-stack[i])/d, &stack[..i]))
}

fn part1(inp: &str) -> u64 {
    inp.lines().map(|l| {
        let (target, stack) = l.split(':').map(|x| x.trim()).collect_tuple().unwrap();
        let target = target.parse::<u64>().unwrap();
        let stack = stack.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect_vec();
        (target, stack)
    }).filter(|(t, s)| test(*t, s))
    .map(|(t, _)| t)
    .sum()
}

#[cfg(test)]
mod benching {
    use aoc24::load_input;
    use test::Bencher;

    use crate::part1;

    #[bench]
    fn bench_day7(b: &mut Bencher) {
        let inp = load_input("day7");
        b.iter(|| part1(&inp));
    }
}

fn main() {
    let inp = load_input("day7");
    println!("{}", part1(&inp));
}