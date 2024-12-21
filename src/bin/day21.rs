use core::str;
use std::{fmt::Display, iter::once};

use aoc24::load_input;

use itertools::Itertools;
use phf::{phf_map, Map};
use smallvec::SmallVec;

static NUMERIC_PAD: Map<u8, (i8, i8)> = phf_map! {
    b'0' => (1, 0),
    b'A' => (2, 0),
    b'1' => (0, 1),
    b'2' => (1, 1),
    b'3' => (2, 1),
    b'4' => (0, 2),
    b'5' => (1, 2),
    b'6' => (2, 2),
    b'7' => (0, 3),
    b'8' => (1, 3),
    b'9' => (2, 3)
};
static DIRECTION_PAD: Map<u8, (i8, i8)> = phf_map! {
    b'<' => (0, 0),
    b'v' => (1, 0),
    b'>' => (2, 0),
    b'^' => (1, 1),
    b'A' => (2, 1)
};

fn solve(keypad: &Map<u8, (i8, i8)>, target: &[u8]) -> Vec<u8> {
    let mut pos = *keypad.get(&b'A').unwrap();
    let mut control = *DIRECTION_PAD.get(&b'A').unwrap();

    let mut commands = vec![];
    
    let mut candidates = SmallVec::<[u8; 2]>::new();
    for x in target {
        let pos2 = *keypad.get(x).unwrap();
        while pos != pos2 {
            candidates.clear();
            if pos2.0 > pos.0 {candidates.push(b'>');}
            if pos2.0 < pos.0 {candidates.push(b'<');}
            if pos2.1 > pos.1 {candidates.push(b'^');}
            if pos2.1 < pos.1 {candidates.push(b'v');}
            let chosen = candidates.iter().min_by_key( // select best move by manhattan dist to pointer
                |c| {
                let (x2, y2) = *DIRECTION_PAD.get(c).unwrap();
                x2.abs_diff(control.0) + y2.abs_diff(control.1)
            }).unwrap();
            let k;
            match *chosen {
                b'>' => {k = pos2.0 - pos.0; pos.0 = pos2.0;}
                b'<' => {k = pos.0 - pos2.0; pos.0 = pos2.0;}
                b'^' => {k = pos2.1 - pos.1; pos.1 = pos2.1;}
                b'v' => {k = pos.1 - pos2.1; pos.1 = pos2.1;}
                _ => unreachable!()
            }
            commands.extend(once(chosen).cycle().take(k as usize));
            control = *DIRECTION_PAD.get(chosen).unwrap();
        }
        commands.push(b'A');
    }

    commands
}

fn part1(inp: &str) -> impl Display {
    inp.lines().map(|l| {
        let a = solve(&NUMERIC_PAD, l.as_bytes());
        let b = solve(&DIRECTION_PAD, &a);
        let c = solve(&DIRECTION_PAD, &b);
        dbg!(str::from_utf8(&c).unwrap());
        let n = l[..=l.chars().enumerate().take_while(|&(_, x)| x.is_digit(10))
            .last().unwrap().0].parse::<usize>().unwrap();
        println!("{}*{}={}", c.len(), n, c.len() * n);
        n*c.len()
    }).sum::<usize>()
}

fn main() {
    let inp = load_input("day21");
    println!("{}", part1(&inp));
}