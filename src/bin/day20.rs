use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use aoc24::load_input;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn part1(inp: &str) -> impl Display {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let (sx, sy) = (0..x_len).cartesian_product(0..y_len).filter(|&(x,y)| grid[y][x] == 'S').next().unwrap();
    let (ex, ey) = (0..x_len).cartesian_product(0..y_len).filter(|&(x,y)| grid[y][x] == 'E').next().unwrap();

    let mut queue = vec![];
    queue.push((sx, sy, 0));
    let mut distances = vec![vec![u64::MAX; x_len]; y_len];
    while let Some((x, y, score)) = queue.pop() {
        if distances[y][x] < score {continue};
        distances[y][x] = score; 
        for (x2, y2) in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)] {
            if grid[y2][x2] == '#' {continue};
            queue.push((x2, y2, score+1));
        }
    }

    let mut total = 0;
    for x in 0..x_len {
        for y in 0..y_len {
            if grid[y][x] != '#' && distances[y][x] < u64::MAX {
                for x2 in x.saturating_sub(22)..(x+22).min(x_len) {
                    for y2 in y.saturating_sub(22)..(y+22).min(y_len) {
                        let cheat_time = x.abs_diff(x2)+y.abs_diff(y2);
                        if x.abs_diff(x2)+y.abs_diff(y2) > 20 {continue};
                        if grid[y2][x2] != '#' && distances[y][x] >= 100 + distances[y2][x2] + cheat_time as u64 {
                            // println!("Jump {x} {y} -> {x2} {y2} {}", distances[y][x] - distances[y2][x2] - cheat_time as u64);
                            total += 1;
                        }
                    }   
                }
            }
        }
    }

    // for y in 0..y_len {
    //     for x in 0..x_len {
    //         if grid[y][x] == '#' {print!("#  "); continue}
    //         print!("{:02} ", distances[y][x]);
    //     }
    //     println!();
    // }

    println!("{}", distances[ey][ex]);
    total
}
// < 15268
// < 1344
// > 599
fn part2(inp: &str) -> impl Display {
    0
}
// > 970177

fn main() {
    let inp = load_input("day20");
    println!("{}", part1(&inp));
}