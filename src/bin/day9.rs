#![allow(unused)]
#![feature(let_chains)]

use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) -> u64 {
    let mut files = inp.chars().chain((inp.len() % 2 == 1).then_some('0'))
        .map(|c| (c as u8 - '0' as u8) as usize).chunks(2).into_iter()
        .enumerate().map(|(i, mut c)|
            itertools::repeat_n(Some(i as u64), c.next().unwrap()).chain(itertools::repeat_n(None, c.next().unwrap()))
        )
        .flatten().collect_vec();
    let mut i = 0;
    let mut j = files.len() - 1;
    'swap: loop {
        if i >= j {break};
        while files[i].is_some() {
            i += 1; if i >= j {break 'swap}
        }
        (files[i], files[j]) = (files[j], files[i]);
        while j > i {
            j -= 1; if files[j].is_some() {break};
        }
    }
    files.into_iter().enumerate().filter_map(|(i, x)| x.and_then(|x| Some(x*i as u64))).sum()
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum System {File, Space}
// use System::*;


// fn part2(inp: &str) -> u64 {
//     let mut files = inp.chars().chain((inp.len() % 2 == 1).then_some('0'))
//         .map(|c| (c as u8 - '0' as u8) as usize).chunks(2).into_iter()
//         .enumerate().map(|(i, mut c)|
//             [(File, c.next().unwrap()), (Space, c.next().unwrap())]
//         )
//         .flatten().collect_vec();
//     let mut j = files.len() - 2;
//     assert!(files[j].0 == File);
//     let mut k = files[j].1;
//     loop {
//         let n = files[j].1;
//         if let Some(i) = files.iter().position(|x| x.0 == Space && x.1 >= n) {
//             (files[i], files[j]) = (files[j], files[i]);
//             if files[j].1 > n {
//                 files.insert(i+1, (Space, files[j].1-n));
//             }
//         }
//         j -= 2;
//         k -= 1;
//         if files[j].1 != k {break};
//     }
//     println!("{files:?}");
//     files.into_iter().enumerate().filter(|(i, x)| x.0 == File).map(|(i, x)| i as u64*x.1 as u64).sum()
// }

fn part3(inp: &str) -> u64 {
    let mut files = inp.chars().chain((inp.len() % 2 == 1).then_some('0'))
        .map(|c| (c as u8 - '0' as u8) as usize).chunks(2).into_iter()
        .enumerate().map(|(i, mut c)|
            itertools::repeat_n(Some(i as u64), c.next().unwrap()).chain(itertools::repeat_n(None, c.next().unwrap()))
        )
        .flatten().collect_vec();
    let mut k = files.iter().filter_map(|x| *x).rev().next().unwrap();
    'swap: loop {
        // for f in &files {
        //     match f {
        //         Some(x) => print!("{x}"),
        //         None => print!(".")
        //     }
        // }
        // println!();
        let a = match files.iter().position(|x| x == &Some(k)) {
            Some(a) => a,
            None => break
        };
        let b = files.len() - files.iter().rev().position(|x| x == &Some(k)).unwrap();
        if let Some(c) = (0..files.len()-b+a).filter(|i| files[*i..(*i+b-a)].iter().all(|x| x == &None))
            .next() && c < a {
                for i in 0..(b-a) {
                    (files[c+i], files[a+i]) = (files[a+i], files[c+i]);
                }
            }
        if k == 0 {break};
        k -= 1;
    }
    files.into_iter().enumerate().filter_map(|(i, x)| x.and_then(|x| Some(x*i as u64))).sum()
}

fn main() {
    let inp = load_input("day9");
    println!("{}", part3(&inp));
}