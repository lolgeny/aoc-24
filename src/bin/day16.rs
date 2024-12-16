use std::{collections::{BinaryHeap, HashSet}, fmt::Display};

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
    let visited = {
        let mut visited = HashSet::new(); visited.insert((sx, sy)); visited
    };
    queue.push(VisitedNode {node: Node { fwd: 0, turns: 0, x: sx, y: sy, dx: 1, dy: 0 }, visited});
    let mut overall_visited = HashSet::new();
    while let Some(VisitedNode {node: n, mut visited}) = queue.pop() {
        if overall_visited.is_superset(&visited) {continue}
        dbg!(n); // 400 fwd + 72
        if n.turns > opt_turns {continue}
        if n.fwd > opt_fwd {continue}
        let others = [(-1,0),(1,0),(0,1),(0,-1)].into_iter().filter(|&(dx, dy)| {
            if dx == -n.dx && dy == -n.dy {return false};
            dx != n.dx && dy != n.dy
        }).collect_vec();
        let (mut x, mut y) = (n.x, n.y);
        let mut dfwd = 0;
        while grid[y][x] != '#' {
            if grid[y][x] == 'E'   && n.fwd+dfwd == opt_fwd /*&& n.turns == opt_turns */{
                // println!("{}", n.fwd+dfwd);
                println!("OptImal");
                overall_visited.extend(visited); break;
            };
            for &(dx, dy) in &others {
                let x2 = x as isize + dx;
                let y2 = y as isize + dy;
                if x2 < 0 || y2 < 0 || x2 as usize >= x_len || y2 as usize >= y_len {continue};
                let mut new_visited = visited.clone();
                new_visited.insert((x2 as usize, y2 as usize));
                if !visited.contains(&(x2 as usize,y2 as usize)) && grid[y2 as usize][x2 as usize] != '#' {
                    queue.push(VisitedNode{node: Node {fwd: n.fwd + dfwd + 1, turns: n.turns + 1, x: x2 as usize, y: y2 as usize, dx, dy},
                        visited: new_visited});
                }
            }

            let x2 = (x as isize+n.dx) as usize; let y2 = (y as isize+n.dy) as usize;
            if !visited.insert((x2, y2)) {break};
            x = x2; y = y2; dfwd += 1;
        }
    }
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '#' {print!("#")}
            else if overall_visited.contains(&(x,y)) {print!("O")}
            else {print!(".")}
        }
        println!();
    }
    overall_visited.len()
}

fn main() {
    let inp = load_input("day16");
    println!("{}", part1(&inp));
}