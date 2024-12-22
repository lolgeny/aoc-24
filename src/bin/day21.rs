use core::str;
use std::{collections::BinaryHeap, fmt::Display, iter::once};

use aoc24::load_input;

use itertools::Itertools;
use phf::{phf_map, phf_set, Map, Set};
use rand::{thread_rng, Rng};
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
    let mut rng = thread_rng();
    for x in target {
        let pos2 = *keypad.get(x).unwrap();
        let mut tabu = None;
        while pos != pos2 {
            candidates.clear();
            if pos2.1 < pos.1 {candidates.push(b'v');}
            if pos2.1 > pos.1 {candidates.push(b'^');}
            if pos2.0 > pos.0 {candidates.push(b'>');}
            if pos2.0 < pos.0 {candidates.push(b'<');}
            // let chosen = *candidates.iter().filter(|x| Some(**x) != tabu).min_by_key( // select best move by manhattan dist to pointer
            //     |c| {
            //     let (x2, y2) = *DIRECTION_PAD.get(c).unwrap();
            //     x2.abs_diff(control.0) + y2.abs_diff(control.1)
            // }).unwrap();
            let chosen = candidates[rng.gen_range(0..candidates.len())];
            // let chosen = candidates[0];
            let mut pos3 = pos;
            match chosen {
                b'>' => {pos3.0 += 1;}
                b'<' => {pos3.0 -= 1;}
                b'^' => {pos3.1 += 1;}
                b'v' => {pos3.1 -= 1;}
                _ => unreachable!()
            }
            if !keypad.values().contains(&pos3) {
                tabu = Some(chosen);
                continue};
            tabu = None;
            pos = pos3;
            commands.push(chosen);
            control = *DIRECTION_PAD.get(&chosen).unwrap();
        }
        commands.push(b'A');
    }

    commands
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Node {
    heur: i64,
    path: Vec<u8>,
    a: (i8, i8),
    b: (i8, i8),
    c: (i8, i8),
    progress: usize
}
impl Node {
    pub fn calc(path: Vec<u8>, a: (i8, i8), b: (i8, i8), c: (i8, i8), progress: usize) -> Self {
        Self { heur: progress as i64-(path.len() as i64), path, a, b, c, progress }
    }
}

fn direction(dir: u8) -> (i8, i8) {
    match dir {
        b'^' => (0, 1),
        b'v' => (0, -1),
        b'<' => (-1, 0),
        b'>' => (1, 0),
        _ => panic!()
    }
}

fn solve2(target: &[u8]) -> Vec<u8> {
    let n0 = *NUMERIC_PAD.get(&b'A').unwrap();
    let d0 = *DIRECTION_PAD.get(&b'A').unwrap();
    
    let mut queue = BinaryHeap::new();
    queue.push(Node::calc(Vec::new(), n0, d0, d0, 0));
    while let Some(n) = queue.pop() {
        if n.progress == target.len() {return n.path};
        for dir in [b'^', b'v', b'<', b'>'] {
            let (dx, dy) = direction(dir);
            let c2 = (n.c.0 + dx, n.c.1 + dy);
            let _c_cmd = match c2 {
                (0, 0) => b'<',
                (1, 0) => b'v',
                (2, 0) => b'>',
                (1, 1) => b'^',
                (2, 1) => b'A',
                _ => continue
            };
            let mut path2 = n.path.clone(); path2.push(dir);
            queue.push(Node::calc(path2.clone(), n.a, n.b, c2,n.progress));
        }
        let mut path2 = n.path.clone();
        path2.push(b'A');
        // press c!
        let c_cmd = match n.c {
            (0, 0) => b'<',
            (1, 0) => b'v',
            (2, 0) => b'>',
            (1, 1) => b'^',
            (2, 1) => b'A',
            _ => continue
        };
        if c_cmd == b'A' {
            let b_cmd = match n.b {
                (0, 0) => b'<',
                (1, 0) => b'v',
                (2, 0) => b'>',
                (1, 1) => b'^',
                (2, 1) => b'A',
                _ => continue
            };
            if b_cmd == b'A' {
                if n.a == *NUMERIC_PAD.get(&target[n.progress]).unwrap() {
                    println!("woosh {} {}", n.progress, queue.len());
                    queue.push(Node::calc(path2, n.a, n.b, n.c, n.progress+1));
                }
            } else {
                let (dx, dy) = direction(b_cmd);
                let a2 = (n.a.0 + dx, n.a.1 + dy);
                if !NUMERIC_PAD.values().contains(&a2) {continue};
                queue.push(Node::calc(path2, a2, n.b, n.c, n.progress));
            }
        } else {
            let (dx, dy) = direction(c_cmd);
            let b2 = (n.b.0 + dx, n.b.1 + dy);
            let _b_cmd = match b2 {
                (0, 0) => b'<',
                (1, 0) => b'v',
                (2, 0) => b'>',
                (1, 1) => b'^',
                (2, 1) => b'A',
                _ => continue
            };
            queue.push(Node::calc(path2, n.a, b2, n.c, n.progress));
        }
    }
    todo!()
}

fn part1(inp: &str) -> impl Display {
    inp.lines().map(|l| {
        let mut opt_c = vec![]; let mut opt_len = usize::MAX;
        for _ in 0..100 {
            let mut a = solve(&NUMERIC_PAD, l.as_bytes());
            for i in 0..2 {
                // println!("solving {i}, a is {}", a.len());
                a = solve(&DIRECTION_PAD, &a);
            }
            if a.len() < opt_len {
                opt_len = a.len(); opt_c = a;
            }
        }
        let c = opt_c;
        dbg!(str::from_utf8(&c).unwrap());
        // let c = solve2(l.as_bytes());
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