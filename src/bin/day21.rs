#![feature(let_chains)]

use core::str;
use std::{cell::RefCell, collections::{BinaryHeap, VecDeque}, fmt::Display, iter::once, sync::RwLock};

use aoc24::load_input;

use hashbrown::{HashMap, HashSet};
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

static DXY: Map<u8, (i8, i8)> = phf_map! {
    b'<' => (-1, 0),
    b'v' => (0, -1),
    b'>' => (1, 0),
    b'^' => (0, 1)
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
            let chosen = *candidates.iter().filter(|x| Some(**x) != tabu).min_by_key( // select best move by manhattan dist to pointer
                |c| {
                let (x2, y2) = *DIRECTION_PAD.get(c).unwrap();
                x2.abs_diff(control.0) + y2.abs_diff(control.1)
            }).unwrap();
            // let chosen = candidates[rng.gen_range(0..candidates.len())];
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

fn solve2(target: &[u8], ti: usize, pos: (i8, i8), level: u8,
    cache: &mut HashMap<(usize, (i8, i8)), (u64, HashSet<VecDeque<u8>>)>,
    solving: &mut HashSet<(usize, (i8, i8))>
) -> (u64, HashSet<VecDeque<u8>>) {
    // println!("{ti},");
    if let Some(x) = cache.get(&(ti, pos)) {return x.clone()};
    if solving.contains(&(ti, pos)) {return (u64::MAX, HashSet::new())};
    solving.insert((ti, pos));
    if ti == target.len() {return (0, HashSet::from([VecDeque::new()]))};
    let keypad = if level == 0 {&NUMERIC_PAD} else {&DIRECTION_PAD};

    let target_pos = *keypad.get(&target[ti]).unwrap();
    if pos == target_pos {
        let (mut score, mut path) = solve2(target, ti+1, pos, level, cache, solving);
        score = score.saturating_add(1);
        path = path.into_iter().map(|mut x| {x.push_front(b'A'); x}).collect();
        cache.insert((ti, pos), (score, path.clone()));
        (score, path)
    } else {
        let mut opt = u64::MAX;
        let mut opt_path = HashSet::new();
        
        for d in [b'>', b'<', b'^', b'v'] {
            let (dx, dy) = DXY.get(&d).unwrap();
            let pos2 = (pos.0 + dx, pos.1 + dy);
            if !keypad.values().contains(&pos2) {continue};
            let (mut score, path) = solve2(target, ti, pos2, level, cache, solving);
            score = score.saturating_add(1);
            if score < opt {
                opt = score;
                opt_path = path.into_iter().map(|mut x| {x.push_front(d); x}).collect();
            } else if score == opt {
                opt_path.extend(path.into_iter().map(|mut x| {x.push_front(d); x}));
            }
        }
        cache.insert((ti, pos), (opt, opt_path.clone()));
        (opt, opt_path)
    }

}


fn part1(inp: &str) -> impl Display {
    inp.lines().map(|l| {
        // let mut cache = HashMap::new(); let mut solving = HashSet::new();
        // let (score, mut path) = solve2(l.as_bytes(), 0, *NUMERIC_PAD.get(&b'A').unwrap(), 0, &mut cache, &mut solving);
        // println!("{}: {}", score, str::from_utf8(&path.make_contiguous()).unwrap());


        let mut cache = HashMap::new(); let mut solving = HashSet::new();
        let (mut score, mut path) = solve2(l.as_bytes(), 0, *NUMERIC_PAD.get(&b'A').unwrap(), 0, &mut cache, &mut solving);

        for _ in 0..25 {
            println!("{}", path.len());
            // for mut p in path.clone() {
                // print!("{}:: ", p.len());
                // println!("{}", str::from_utf8(p.make_contiguous()).unwrap());
            // }
            // println!("REPEAT");
            let mut path2 = HashSet::new();
            let mut opt_score = u64::MAX;
            for mut p in path {
                let mut cache = HashMap::new(); let mut solving = HashSet::new();
                let (score2, path3) = solve2(&p.make_contiguous(), 0, *DIRECTION_PAD.get(&b'A').unwrap(), 1, &mut cache, &mut solving);
                if score2 < opt_score {
                    path2 = path3; opt_score = score2;
                } else if score2 == opt_score {
                    path2.extend(path3);
                }
            }
            path = path2;
            score = opt_score;
        }


        // for mut p in path {
            // print!("{}:: ", p.len());
            // println!("{}", str::from_utf8(p.make_contiguous()).unwrap());
        // }
        dbg!(score)
    }).sum::<u64>()
}

// <v<A>A< A>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// <v  A<A A>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A


fn solve3(cache: &mut HashMap<(u8, u8, u8), u64>, start: u8, end: u8, level: u8, target_level: u8) -> u64 {
    if level == target_level {return 1};
    // println!("Call: level = {level}   from {} to {}", start as char, end as char);
    if level > 0 && let Some(x) = cache.get(&(start, end, level)) {return *x};
    // if start == end {return 1};
    let pad = if level == 0 {&NUMERIC_PAD} else {&DIRECTION_PAD};
    let (x1, y1) = *pad.get(&start).unwrap();
    let (mut x, mut y) = (x1, y1);
    let (x2, y2) = *pad.get(&end).unwrap();
    let mut path = vec![];
    while x < x2 { path.push(b'>'); x += 1; }
    while x > x2 { path.push(b'<'); x -= 1; }
    while y < y2 { path.push(b'^'); y += 1; }
    while y > y2 { path.push(b'v'); y -= 1; }
    let p_len = path.len();
    let mut visited = HashSet::new();

    let mut best_cost = u64::MAX;
    'perms: for mut p in path.into_iter().permutations(p_len) {
        p.push(b'A');
        if visited.contains(&p) {continue};
        visited.insert(p.clone());
        let mut pos = (x1, y1);
        for x in &p {
            if *x == b'A' {break};
            let (dx, dy) = DXY.get(x).unwrap();
            pos.0 += *dx; pos.1 += *dy;
            if !pad.values().contains(&pos) {continue 'perms};
        }
        let mut cost = 0u64;
        let mut pos = b'A';
        // println!("Level {level} following {}", str::from_utf8(&p).unwrap());
        for x in p {
            cost = cost.saturating_add(solve3(cache, pos, x, level + 1, target_level));
            pos = x;
        }
        // println!("got {cost}");
        best_cost = best_cost.min(cost);
    }
    // println!("RETURN: level = {level}   from {} to {}", start as char, end as char);
    // println!("Overall: {best_cost}");

    if level > 0 {
        cache.insert((start, end, level), best_cost);
    }
    best_cost
}

fn part2(inp: &str) -> impl Display {
    inp.lines().map(|l| {
        // println!("{l}");
        let mut pos = b'A';
        let mut cost = 0u64;
        let mut cache = HashMap::new();
        for &x in l.as_bytes() {
            cost = cost.saturating_add(solve3(&mut cache, pos, x, 0, 26));
            pos = x;
        }
        let n = l.chars().take_while(|x| x.is_digit(10)).collect::<String>().parse::<u64>().unwrap();
        dbg!(cost);
        n*cost
    }).sum::<u64>()
}

fn main() {
    let inp = load_input("day21");
    println!("{}", part2(&inp));
}