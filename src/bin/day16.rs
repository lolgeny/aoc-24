#![allow(unused)]
use std::{collections::{BinaryHeap, HashMap, HashSet}, fmt::Display};

use aoc24::load_input;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Node {
    fwd: u64, turns: u64,
    x: usize, y: usize, dx: isize, dy: isize
}
impl Node {
    fn heuristic(&self) -> i64 {
        self.fwd as i64 + self.turns as i64 * 1000
        + self.y as i64 - self.x as i64
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic() == other.heuristic()
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heuristic().partial_cmp(&self.heuristic())
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic().cmp(&self.heuristic())
    }
}

#[derive(Debug, Clone)]
struct VisitedNode {
    node: Node, visited: HashSet<(usize, usize)>
}
impl PartialEq for VisitedNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.eq(&other.node)
    }
}
impl Eq for VisitedNode {}
impl PartialOrd for VisitedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.node.partial_cmp(&other.node)
    }
}
impl Ord for VisitedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.cmp(&other.node)
    }
}

fn part1(inp: &str) -> impl Display {
    let grid = inp.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let (sx, sy) = (1, y_len-2);
    let (ex, ey) = (x_len-2, 1);
    
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert((sx, sy), (0, 0, 1, 0));
    queue.extend([
        Node { fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 }
    ]);
    'search: while let Some(n) = queue.pop() {
        if !visited.contains_key(&(n.x, n.y)) {
            visited.insert((n.x, n.y), (n.fwd, n.turns, n.dx, n.dy));
        }

        let others = [(-1,0),(1,0),(0,1),(0,-1)].into_iter().filter(|&(dx, dy)| {
            if dx == -n.dx && dy == -n.dy {return false};
            dx != n.dx && dy != n.dy
        }).collect_vec();
        let (mut x, mut y) = (n.x, n.y);
        let mut dfwd = 0;
        while grid[y][x] != '#' {
            if (x,y)==(ex,ey) {return 1000*n.turns + n.fwd + dfwd};
            for &(dx, dy) in &others {
                let x2 = x as isize + dx;
                let y2 = y as isize + dy;
                if x2 < 0 || y2 < 0 || x2 as usize >= x_len || y2 as usize >= y_len {continue};
                if !visited.contains_key(&(x2 as usize,y2 as usize)) && grid[y2 as usize][x2 as usize] != '#' {
                    queue.push(Node {fwd: n.fwd + dfwd + 1, turns: n.turns + 1, x: x2 as usize, y: y2 as usize, dx, dy});
                }
            }

            let x2 = (x as isize+n.dx) as usize; let y2 = (y as isize+n.dy) as usize;
            if visited.contains_key(&(x2, y2)) {break};
            visited.insert((x2, y2), (n.fwd+dfwd+1, n.turns, n.dx, n.dy));
            x = x2; y = y2; dfwd += 1;
        }
    }
    panic!("No path")
}

fn part2(inp: &str) -> impl Display {
    const DXY: [(isize, isize); 4] = [(-1,0),(1,0),(0,1),(0,-1)];
    let grid = inp.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len(); let y_len = grid.len();
    let (sx, sy) = (1, y_len-2);
    let (ex, ey) = (x_len-2, 1);
    
    fn search(grid: &[Vec<char>], x_len: usize, y_len: usize, sx: usize, sy: usize, ex: usize, ey: usize) -> HashMap<(usize, usize, isize, isize), (u64, u64)> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        visited.insert((sx, sy, 1, 0), (0, 0));
        queue.extend([
            Node { fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 }
        ]);
        'search: while let Some(n) = queue.pop() {
            if !visited.contains_key(&(n.x, n.y, n.dx, n.dy)) {
                visited.insert((n.x, n.y, n.dx, n.dy), (n.fwd, n.turns));
            }
            for (dx, dy) in DXY {
                if (dx, dy) == (n.dx, n.dy) {continue};
                if !visited.contains_key(&(n.x,n.y,dx,dy)) {
                    queue.push(Node { fwd: n.fwd, turns: n.turns+1, x: n.x, y: n.y, dx, dy });
                }
            }
    
            let others = DXY.into_iter().filter(|&(dx, dy)| {
                if dx == -n.dx && dy == -n.dy {return false};
                dx != n.dx && dy != n.dy
            }).collect_vec();
            let (mut x, mut y) = (n.x, n.y);
            let mut dfwd = 0;
            while grid[y][x] != '#' {
                if (x, y) == (ex, ey) {continue 'search};
                for &(dx, dy) in &others {
                    let x2 = x as isize + dx;
                    let y2 = y as isize + dy;
                    if x2 < 0 || y2 < 0 || x2 as usize >= x_len || y2 as usize >= y_len {continue};
                    if !visited.contains_key(&(x2 as usize,y2 as usize,dx,dy)) && grid[y2 as usize][x2 as usize] != '#' {
                        queue.push(Node {fwd: n.fwd + dfwd + 1, turns: n.turns + 1, x: x2 as usize, y: y2 as usize, dx, dy});
                    }
                }
    
                let x2 = (x as isize+n.dx) as usize; let y2 = (y as isize+n.dy) as usize;
                if visited.contains_key(&(x2, y2, n.dx, n.dy)) {break};
                visited.insert((x2, y2, n.dx, n.dy), (n.fwd+dfwd+1, n.turns));
                x = x2; y = y2; dfwd += 1;
            }
        }
        visited
    }
    let distances1 = search(&grid, x_len, y_len, sx, sy, ex, ey);
    let distances2 = search(&grid, x_len, y_len, ex, ey, sx, sy);

    let (opt_fwd, opt_turns) = *DXY.into_iter().filter_map(|(dx, dy)| distances1.get(&(ex,ey,dx,dy))).min().unwrap();

    // for y in 0..y_len {
    //     for x in 0..x_len {
    //         if grid[y][x] == '#' {print!("## ");}
    //         else if let Some(d) = distances1.get(&(x,y)) {
    //             print!("{:02} ", d.0);
    //         } else {
    //             print!(".  ");
    //         }
    //     }
    //     println!();
    // }
    // println!("\n");
    // for y in 0..y_len {
    //     for x in 0..x_len {
    //         if grid[y][x] == '#' {print!("## ");}
    //         else if let Some(d) = distances2.get(&(x,y)) {
    //             print!("{:02} ", d.0);
    //         } else {
    //             print!(".  ");
    //         }
    //     }
    //     println!();
    // }
    println!("\n");
    for y in 0..y_len {
        'coords: for x in 0..x_len {
            if grid[y][x] == '#' {print!("#"); continue}
            for (dx, dy) in DXY {
                if let Some(&(a,b)) = distances1.get(&(x,y,dx,dy)) {
                    if let Some(&(c,d)) = distances2.get(&(x,y,dx,dy)) {
                        if a+c == opt_fwd && b+d <= opt_turns {print!("O"); continue 'coords;}
                    }
                }
            }
            print!(".");
        }
        println!();
    }

    // (0..x_len).cartesian_product(0..y_len).filter(|&(x,y)| {
    //     let (a, b) = match distances1.get(&(x,y)) {
    //         Some(x) => *x,
    //         None => {return false}
    //     };
    //     let (c, d) = match distances2.get(&(x,y)) {
    //         Some(x) => *x,
    //         None => {return false}
    //     };
    //     a+c == opt_fwd && b+d <= opt_turns+1
    // }).count()
    0
}

fn main() {
    let inp = load_input("day16");
    println!("{}", part2(&inp));
}
// > 401