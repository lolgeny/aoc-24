use std::{collections::HashSet, io::stdin, ops::Range};

use aoc24::load_input;
use itertools::Itertools;
use regex::Regex;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn count_quadrant(robots: &[(i64, i64, i64, i64)], xs: Range<i64>, ys: Range<i64>) -> usize {
    robots.iter()
    .filter(|&(x, y, _, _)| xs.contains(x) && ys.contains(y))
    .count()
}
fn count_quads(robots: &[(i64, i64, i64, i64)]) -> (usize, usize, usize, usize) {
    (count_quadrant(robots, 0..(WIDTH-1)/2, 0..(HEIGHT-1)/2),
    count_quadrant(robots, (WIDTH+1)/2..WIDTH, 0..(HEIGHT-1)/2),
    count_quadrant(robots, 0..(WIDTH-1)/2, (HEIGHT+1)/2..HEIGHT),
    count_quadrant(robots, (WIDTH+1)/2..WIDTH, (HEIGHT+1)/2..HEIGHT))
}

fn part1(inp: &str) -> usize {
    let re = Regex::new(r"(-)?\d+").unwrap();
    let mut robots: Vec<(i64, i64, i64, i64)> = re.find_iter(&inp).chunks(4)
        .into_iter().map(|c| c
            .map(|x| x.as_str().parse().unwrap()).collect_tuple().unwrap())
        .collect_vec();

    let stdin = stdin();
    for it in 1..=1000000 {
        robots.iter_mut().for_each(|&mut (ref mut x, ref mut y, dx, dy)| {
            *x += dx;
            *y += dy;
            while *x < 0 {*x += WIDTH;}
            while *y < 0 {*y += HEIGHT;}
            while *x >= WIDTH {*x -= WIDTH;}
            while *y >= HEIGHT {*y -= HEIGHT;}
        });
        let positions: HashSet<(i64, i64)> = HashSet::from_iter(robots.iter().map(|&(x, y, _, _)| (x, y)));
        // let (a, b, c, d) = count_quads(&robots);
        // if a.abs_diff(b) > 1 || c.abs_diff(d) > 1 {continue;}
        // if a != b || a != c || a != d {continue};
        // if a != b || c != d {continue};
        // if c + d > 100 {continue};
        // if positions.iter().filter(|(x, y)|
            // !positions.contains(&(WIDTH-1-*x, *y))
        // ).count() > 400 {continue};
        if positions.len() < 500 {continue};
        println!("After {it} seconds...");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if robots.iter().any(|(rx, ry, _, _)| *rx==x && *ry==y) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("\n\n");
        let mut _buf = String::new();
        let _ = stdin.read_line(&mut _buf);
    }

    todo!()
}


// definitely > 180
// seems to be >544
// less than a million
fn main() {
    let inp = load_input("day14");
    println!("{}", part1(&inp));
}