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
            visited.insert((x2, y2), (n.fwd+dfwd+1, n.turns, n.dx, n.dy));
            x = x2; y = y2; dfwd += 1;
        }
    }
    let distances = visited; // remove mutability
    let (e_fwd, e_turns, ..) = *distances.get(&(ex, ey)).unwrap();

    let mut valid = HashSet::new();
    for (x0, y0) in (0..x_len).cartesian_product(0..y_len) {
        if grid[y0][x0] == '#' {continue};

        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let d0 = *distances.get(&(x0, y0)).unwrap();
        visited.insert((x0, y0), d0);
        queue.extend([
            Node { fwd: d0.0, turns: d0.1, x: x0, y: y0, dx: d0.2, dy: d0.3 }
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
                if grid[y][x] == 'E' {break 'search};
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
                // if x2 == ex - 1 && y2 == ey {
                    // println!("{}", n.fwd+dfwd+1);
                // }
                visited.insert((x2, y2), (n.fwd+dfwd+1, n.turns, n.dx, n.dy));
                x = x2; y = y2; dfwd += 1;
            }
        }
        if let Some(&(fwd, turns, ..)) = visited.get(&(ex, ey)) {
            if (fwd, turns) == (e_fwd, e_turns) {
                valid.insert((x0,y0));
            }
        }
    }


    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("## ")}
            // else if valid.contains(&(x,y)) {print!("O")}
            // else {print!(".");}
            else {print!("{:02} ", distances.get(&(x,y)).copied().unwrap_or((99,99,0,0)).0)}
        }
        println!();
    }
    println!("\n\n");
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("## ")}
            // else if valid.contains(&(x,y)) {print!("O")}
            // else {print!(".");}
            else {print!("{:02} ", distances.get(&(x,y)).copied().unwrap_or((99,99,0,0)).1)}
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