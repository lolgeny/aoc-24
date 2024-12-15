use std::{collections::HashMap, fmt::Display};

use aoc24::load_input;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Object {
    Wall, Empty, Box
}
use Object::*;
impl Object {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Wall,
            '.' | '@' => Empty,
            'O' => Box,
            _ => panic!("Unknown char {c}")
        }
    }
}

fn print_grid(grid: &HashMap<(isize, isize), Object>, x_len: isize, y_len: isize, rx: isize, ry: isize) {
    for y in 0..y_len {
        for x in 0..x_len {
            if (x, y) == (rx, ry) {print!("@"); continue}
            print!("{}", match *grid.get(&(x, y)).unwrap() {
                Box => 'O', Wall => '#', Empty => '.'
            });
        }
        println!();
    }
}

fn part1(inp: &str) -> impl Display {
    let (grid, movements) = inp.split("\n\n").collect_tuple().unwrap();
    let (x_len, y_len, x0, y0, mut grid) = {
        let chs = grid.lines().map(|x| x.chars().collect_vec()).collect_vec();
        let x_len = chs[0].len(); let y_len = chs.len();
        let (x0, y0) = (0..x_len).cartesian_product(0..y_len).filter(|&(x, y)| chs[y][x] == '@').next().unwrap();
        (x_len as isize, y_len as isize, x0 as isize, y0 as isize,
            (0..x_len).cartesian_product(0..y_len).map(|(x, y)| ((x as isize, y as isize), Object::from_char(chs[y][x])))
            .collect::<HashMap<(isize, isize), Object>>()
        )
    };
    let (mut rx, mut ry) = (x0, y0);
    for m in movements.trim().chars() {
        // println!("Moving {m}");
        // print_grid(&grid, x_len, y_len, rx, ry);
        let (dx, dy) = match m {
            '^' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '\n' => continue,
            _ => panic!("Unknown direction '{m}'")
        };
        let backup_grid = grid.clone();
        let (mut cx, mut cy) = (rx, ry);
        let valid = loop {
            grid.insert((cx+dx, cy+dy),
                *backup_grid.get(&(cx, cy)).unwrap()
            );
            match *backup_grid.get(&(cx+dx, cy+dy)).unwrap() {
                Empty => {break true}
                Box => {}
                Wall => {break false}
            }
            cx += dx; cy += dy;
        };
        if valid {
            grid.insert((rx, ry), Empty);
            rx += dx;
            ry += dy;
        } else {
            grid = backup_grid;
        }
    }
    (0..x_len).cartesian_product(0..y_len).filter(|&(x, y)| grid.get(&(x,y)).unwrap() == &Box)
        .map(|(x, y)| x + 100*y)
        .sum::<isize>()
}

fn main() {
    let inp = load_input("day15");
    println!("{}", part1(&inp));
}