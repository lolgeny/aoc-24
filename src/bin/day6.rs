use std::collections::HashSet;

use aoc24::load_input;
use itertools::Itertools;

#[allow(unused)]
fn part1(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let obstacles: HashSet<(usize, usize)> = (0..x_len).cartesian_product(0..y_len)
        .filter(|(x, y)| grid[*y][*x] == '#')
        .collect();
    let (mut gx, mut gy) = (0..x_len).cartesian_product(0..y_len)
        .filter(|(x, y)| grid[*y][*x] == '^')
        .next().unwrap();
    let (mut dx, mut dy) = (0isize, -1isize);
    let mut visited = HashSet::new();
    visited.insert((gx, gy));

    loop {
        if (gx as isize) < -dx || (gy as isize) < -dy {break};
        let nx = (gx as isize + dx) as usize;
        let ny = (gy as isize + dy) as usize;
        if nx >= x_len || ny >= y_len {break}
        if obstacles.contains(&(nx, ny)) {
            (dx, dy) = (-dy, dx);
        } else {
            gx = nx;
            gy = ny;
            visited.insert((gx, gy));
        }
    }
    visited.len()
}

#[allow(unused)]
fn part2(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let mut obstacles: HashSet<(usize, usize)> = (0..x_len).cartesian_product(0..y_len)
        .filter(|(x, y)| grid[*y][*x] == '#')
        .collect();
    let (gx0, gy0) = (0..x_len).cartesian_product(0..y_len)
        .filter(|(x, y)| grid[*y][*x] == '^')
        .next().unwrap();

    let mut n = 0;
    let mut first_pass = true;
    let mut visited = HashSet::new();
    visited.insert((gx0, gy0, 0isize, -1isize));
    let mut queue = vec![(gx0, gy0, 0isize, -1isize, visited)];
    while let Some((mut gx, mut gy, mut dx, mut dy, mut visited)) = queue.pop() {
        loop {
            if (gx as isize) < -dx || (gy as isize) < -dy {break};
            let nx = (gx as isize + dx) as usize;
            let ny = (gy as isize + dy) as usize;
            if nx >= x_len || ny >= y_len {break}
            if obstacles.contains(&(nx, ny)) {
                (dx, dy) = (-dy, dx);
            } else {
                if first_pass {
                    queue.push((gx, gy, -dy, dx, visited.clone()));
                }
                gx = nx;
                gy = ny;
                if visited.contains(&(gx, gy, dx, dy)) {
                    n += 1; break;
                }
                visited.insert((gx, gy, dx, dy));
            }
        }
        first_pass = false;
    }

    n
}

fn main() {
    let inp = load_input("day6");
    println!("{}", part2(&inp));
}