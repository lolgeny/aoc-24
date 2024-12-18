#![feature(int_roundings)]

use std::fmt::Display;

use aoc24::load_input;
use itertools::Itertools;

fn execute(prog: &[u64], mut a: i64, mut b: i64, mut c: i64) -> Vec<i64> {
    let mut ip = 0;
    let mut out = vec![];

    'execution: while ip + 1 < prog.len() {
        // println!("A: {a}, B: {b}, C: {c}   ({} {})", prog[ip], prog[ip+1]);
        let op = match prog[ip+1] {
            4 => a,
            5 => b,
            6 => c,
            x => x as i64
        };
        match prog[ip] {
            0 | 6 | 7 => {
                let dv = a.div_floor(2i64.pow(op as u32));
                match prog[ip] {
                    0 => {a = dv;}
                    6 => {b = dv;}
                    7 => {c = dv;}
                    _ => unreachable!()
                }
            }
            1 => {
                b ^= prog[ip+1] as i64;
            }
            2 => {
                b = op % 8;
            }
            3 => {
                if a != 0 {
                    ip = prog[ip+1] as usize; continue 'execution;
                }
            }
            4 => {
                b ^= c;
            }
            5 => { 
                out.push(op % 8);
            }
            x => panic!("Unknown instruction {x}")
        }
        ip += 2;
    }
    out
}

fn part1(inp: &str) -> impl Display {
    let (pre, prog) = inp.split("\n\n").collect_tuple().unwrap();
    let (a, b, c) = pre.split("\n").map(|l| l.split_ascii_whitespace().filter_map(|x| x.parse::<i64>().ok()).next().unwrap())
        .collect_tuple().unwrap();
    let prog = prog.split_ascii_whitespace().skip(1).next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect_vec();

    let out = execute(&prog, a, b, c);

    out.into_iter().map(|x| x.to_string()).join(",")
}

fn solve(prog: &[u64], a0: i64, b: i64, c: i64, target: &[i64]) -> Option<i64> {
    if target.len() == 0 {return Some(a0)};
    let a = a0*8;
    for byte in 0..8 {
        let out = execute(&prog, a+byte, b, c);
        if out[0] == target[0] {
            if let Some(a) = solve(prog, a+byte, b, c, &target[1..]) {
                return Some(a);
            }
        }
    }
    // println!("Success -> {:?}", execute(&prog, a, b, c));
    return None;
}

fn part2(inp: &str) -> impl Display {
    let (pre, prog) = inp.split("\n\n").collect_tuple().unwrap();
    let (_, b, c) = pre.split("\n").map(|l| l.split_ascii_whitespace().filter_map(|x| x.parse::<i64>().ok()).next().unwrap())
        .collect_tuple().unwrap();
    let prog = prog.split_ascii_whitespace().skip(1).next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect_vec();

    let ref_prog = prog.iter().map(|x| *x as i64).rev().collect_vec();
    // let ref_prog = [0,3,3,0,5,5];
    // 2,4,1,2,7,5,1,3,4,3,5,5,0,3,3,0
    for a0 in 0..8192 {
        if let Some(a) = solve(&prog, a0, b, c, &ref_prog) {
            println!("Success -> {:?}", execute(&prog, a, b, c));
            return a;
        }
    }
    println!("Failure");
    0
}

fn part2_clever(_inp: &str) -> i64 {
    // let target = [2,4,1,2,7,5,1,3,4,3,5,5,0,3,3,0];
    let target = [0,3,3,0,5];
    let mut a = 0;
    /*
    b = a & 7 ^ 2
    c = a >> b
    b = !(a & 7)
    b ^= c
    out !(a & 7) ^ (a >> a & 7 ^ 2)
    a /= 8 
    */
    'targets: for x in target {
        println!("Testing {x}...");
        a <<= 3;
        for b in 0..8 {
            let o = !(b & 7) ^ ((a+b) >> (((a+b) & 7) ^ 2));
            if o == x {
                a += b;
                continue 'targets;
            }
        }
        panic!("No valid bit found!");
    }
    a
}

fn main() {
    let inp = load_input("day17");
    println!("{}", part2(&inp));
}