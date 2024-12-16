use std::{collections::{HashMap, HashSet}, fmt::Display};

use aoc24::load_input;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Object {
    Wall, Empty, BoxL, BoxR
}
use Object::*;
impl Object {
    pub fn from_char(c: char) -> (Self, Self) {
        match c {
            '#' => (Wall, Wall),
            '.' | '@' => (Empty, Empty),
            'O' => (BoxL, BoxR),
            _ => panic!("Unknown char {c}")
        }
    }
}

fn print_grid(grid: &HashMap<(isize, isize), Object>, x_len: isize, y_len: isize, rx: isize, ry: isize) {
    for y in 0..y_len {
        for x in 0..x_len {
            if (x, y) == (rx, ry) {print!("@"); continue}
            print!("{}", match *grid.get(&(x, y)).unwrap() {
                BoxL => '[', BoxR => ']', Wall => '#', Empty => '.'
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
        (x_len as isize*2, y_len as isize, x0 as isize*2, y0 as isize,
            (0..x_len).cartesian_product(0..y_len).map(|(x, y)| {
                let (a, b) = Object::from_char(chs[y][x]);
                [((2*x as isize, y as isize), a), ((2*x as isize+1, y as isize), b)]
            })
            .flatten().collect::<HashMap<(isize, isize), Object>>()
        )
    };
    let (mut rx, mut ry) = (x0, y0);
    for m in movements.trim().chars() {
        // print_grid(&grid, x_len, y_len, rx, ry);
        // println!("Moving {m}");
        let (dx, dy) = match m {
            '^' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '\n' => continue,
            _ => panic!("Unknown direction '{m}'")
        };

        let ref_grid = grid.clone();
        let mut fill = HashSet::new();
        let mut queue = vec![(rx+dx, ry+dy)];
        match *grid.get(&(rx+dx, ry+dy)).unwrap() {
            Wall => {continue}
            Empty => {
                rx += dx; ry += dy; continue;
            }
            BoxL if dx == 0 => {queue.push((rx+dx+1, ry+dy))}
            BoxR if dx == 0 => {queue.push((rx+dx-1, ry+dy))}
            _ => {}
        }

        let mut valid = true;
        while let Some((cx, cy)) = queue.pop() {
            println!("{cx}, {cy}");
            if fill.contains(&(cx, cy)) {continue}
            fill.insert((cx, cy));
            match *grid.get(&(cx+dx, cy+dy)).unwrap() {
                Wall => {valid = false; break}
                Empty => {continue}
                BoxL if dx == 0 => {queue.push((cx+1, cy+dy))}
                BoxR if dx == 0 => {queue.push((cx-1, cy+dy))}
                _ => {}
            }
            queue.push((cx+dx, cy+dy));
        }
        if !valid {continue};
        // println!("fill: {fill:?}");
        for &(x, y) in fill.iter() {
            grid.insert((x+dx, y+dy), *ref_grid.get(&(x, y)).unwrap());
            if !fill.contains(&(x-dx, y-dy)) {grid.insert((x,y), Empty);}
        }
        rx += dx;
        ry += dy;
    }
    print_grid(&grid, x_len, y_len, rx, ry);
    (0..x_len).cartesian_product(0..y_len).filter(|&(x, y)| grid.get(&(x,y)).unwrap() == &BoxL)
        .map(|(x, y)| x + 100*y)
        .sum::<isize>()
}

fn main() {
    let inp = load_input("day15");
    println!("{}", part1(&inp));
}