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
    let mut visited = HashSet::new();
    visited.insert((sx, sy));
    queue.push(Node { fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 });
    let mut opt_fwd = 0; let mut opt_turns = 0;
    'search: while let Some(n) = queue.pop() {
        let others = [(-1,0),(1,0),(0,1),(0,-1)].into_iter().filter(|&(dx, dy)| {
            if dx == -n.dx && dy == -n.dy {return false};
            dx != n.dx && dy != n.dy
        }).collect_vec();
        let (mut x, mut y) = (n.x, n.y);
        let mut dfwd = 0;
        while grid[y][x] != '#' {
            if grid[y][x] == 'E' {opt_fwd = n.fwd+dfwd; opt_turns = n.turns; break 'search};
            for &(dx, dy) in &others {
                let x2 = x as isize + dx;
                let y2 = y as isize + dy;
                if x2 < 0 || y2 < 0 || x2 as usize >= x_len || y2 as usize >= y_len {continue};
                if !visited.contains(&(x2 as usize,y2 as usize)) && grid[y2 as usize][x2 as usize] != '#' {
                    queue.push(Node {fwd: n.fwd + dfwd + 1, turns: n.turns + 1, x: x2 as usize, y: y2 as usize, dx, dy});
                }
            }

            let x2 = (x as isize+n.dx) as usize; let y2 = (y as isize+n.dy) as usize;
            if !visited.insert((x2, y2)) {break};
            x = x2; y = y2; dfwd += 1;
        }
    }

    println!("Opt: {opt_fwd} fwd + {opt_turns} turns");
    // do it again
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert((sx, sy), (0, 0));
    queue.extend([
        Node { fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 }
    ]);
    'search: while let Some(n) = queue.pop() {
        if !visited.contains_key(&(n.x, n.y)) {
            visited.insert((n.x, n.y), (n.fwd, n.turns));
        }

        let others = [(-1,0),(1,0),(0,1),(0,-1)].into_iter().filter(|&(dx, dy)| {
            if dx == -n.dx && dy == -n.dy {return false};
            dx != n.dx && dy != n.dy
        }).collect_vec();
        let (mut x, mut y) = (n.x, n.y);
        let mut dfwd = 0;
        while grid[y][x] != '#' {
            if grid[y][x] == 'E' {continue 'search};
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
            if x2 == ex - 1 && y2 == ey {
                println!("{}", n.fwd+dfwd+1);
            }
            visited.insert((x2, y2), (n.fwd+dfwd+1, n.turns));
            x = x2; y = y2; dfwd += 1;
        }
    }
    let distances = visited; // remove mutability

    // and again (todo delete first time)
    // let mut queue = BinaryHeap::new();
    // let mut valid = HashSet::new();
    // let visited = {
    //     let mut visited = HashSet::new();
    //     visited.insert((sx, sy));
    //     visited
    // };
    // let mut global_visited : HashSet<(usize, usize, isize, isize, u64, u64)> = HashSet::new();
    // queue.extend([
    //     VisitedNode { node: Node {fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 }, visited }
    // ]);
    // 'search: while let Some(VisitedNode {node: n, mut visited}) = queue.pop() {
    //     if global_visited.contains(&(n.x, n.y, n.dx, n.dy, n.turns, n.fwd)) {continue};
    //     // if visited.is_subset(&valid) {continue};
    //     // if dbg!(distances.get(&(n.x, n.y))) != dbg!(Some(&(n.fwd, n.turns))) {continue};
    //     let others = [(-1,0),(1,0),(0,1),(0,-1)].into_iter().filter(|&(dx, dy)| {
    //         if dx == -n.dx && dy == -n.dy {return false};
    //         dx != n.dx && dy != n.dy
    //     }).collect_vec();
    //     let (mut x, mut y) = (n.x, n.y);
    //     // println!("Starting {x} {y}");
    //     let mut dfwd = 0;
    //     while grid[y][x] != '#' {
    //         let (f, t) = *distances.get(&(x, y)).unwrap();
    //         // println!("{:?} ?= {:?}  ({x} {y})", distances.get(&(x,y)), (n.fwd+dfwd, n.turns));
    //         // if distances.get(&(x, y)) != Some(&(n.fwd+dfwd, n.turns)) {break};
    //         if n.fwd+dfwd > f  {break};
    //         if grid[y][x] == 'E' {
    //             // println!("solution candidate {} {t} {visited:?}", n.turns);
    //             println!("found solution {}", queue.len());
    //             if n.turns == t {
    //                 valid.extend(visited);
    //             }
    //             continue 'search;
    //         };
    //         for &(dx, dy) in &others {
    //             let x2 = x as isize + dx;
    //             let y2 = y as isize + dy;
    //             if x2 < 0 || y2 < 0 || x2 as usize >= x_len || y2 as usize >= y_len {continue};
    //             if !global_visited.contains(&(x2 as usize,y2 as usize, dx, dy, n.turns+1, n.fwd+dfwd)) && grid[y2 as usize][x2 as usize] != '#' {
    //                 let mut new_visited = visited.clone();
    //                 new_visited.insert((x2 as usize, y2 as usize));
    //                 queue.push(VisitedNode {node: Node {fwd: n.fwd + dfwd + 1, turns: n.turns + 1, x: x2 as usize, y: y2 as usize, dx, dy}, visited: new_visited});
    //             }
    //         }

    //         let x2 = (x as isize+n.dx) as usize; let y2 = (y as isize+n.dy) as usize;
    //         if visited.contains(&(x2, y2)) {break};
    //         visited.insert((x2, y2));
    //         global_visited.insert((x2, y2, n.dx, n.dy, n.turns, n.fwd+dfwd));
    //         x = x2; y = y2; dfwd += 1;
    //     }
    // }

    let mut queue = vec![(ex, ey, vec![(ex, ey)])];
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut valid = HashSet::<(usize,usize)>::new();
    while let Some((x, y, path)) = queue.pop() {
        // visited.insert((x,y));
        let (f, t) = *distances.get(&(x,y)).unwrap();
        if (x, y) == (sx, sy) {
            if t == 0 && f == 0 {
                valid.extend(&path);
            }
            continue;
        };
        for (x2,y2) in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)] {
            if grid[y2][x2] == '#' || visited.contains(&(x2,y2)) {continue};
            let (f2, t2) = *distances.get(&(x2, y2)).unwrap();
            if f2 <= f/* && t2 <= t*/ {
                let mut path2 = path.clone();
                path2.push((x2,y2));
                queue.push((x2,y2,path2));
            }
        }
    }

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("## ")}
            // else if valid.contains(&(x,y)) {print!("O")}
            // else {print!(".");}
            else {print!("{:02} ", distances.get(&(x,y)).copied().unwrap_or((99,99)).0)}
        }
        println!();
    }
    println!("\n\n");
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("## ")}
            // else if valid.contains(&(x,y)) {print!("O")}
            // else {print!(".");}
            else {print!("{:02} ", distances.get(&(x,y)).copied().unwrap_or((99,99)).1)}
        }
        println!();
    }

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("#")}
            else if valid.contains(&(x,y)) {print!("O")}
            else {print!(".");}
            // else {print!("{:02} ", visited.get(&(x,y)).copied().unwrap_or((99,99)).0)}
        }
        println!();
    }
    valid.len()
}

fn main() {
    let inp = load_input("day16");
    println!("{}", part1(&inp));
}
// > 401