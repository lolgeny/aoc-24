use std::fmt::Display;

use aoc24::load_input;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn part1(inp: &str) -> impl Display {
    let graph = inp.lines().map(|l| {
        let (a, b) = l.split('-').collect_tuple().unwrap();
        [(a, b), (b, a)]
    }).flatten().collect::<HashSet<(&str, &str)>>();
    let mut adjacent: HashMap<&str, Vec<&str>> = HashMap::new();
    for g in &graph {
        if let Some(a) = adjacent.get_mut(&g.0) {
            a.push(g.1);
        } else {
            adjacent.insert(g.0, vec![g.1]);
        }
    }
    let mut triangles = HashSet::new();
    for g in &graph {
        for &x in adjacent.get(g.0).unwrap() {
            if x == g.1 {continue}
            if graph.contains(&(g.1, x)) {
                let mut tri = [g.0, g.1, x];
                tri.sort();
                triangles.insert(tri);
            }
        }
    }
    triangles.into_iter().filter(|t| t.iter().any(|x| x.starts_with('t'))).count()
}

fn max_clique<'a>(mut r: Vec<&'a str>, mut p: HashSet<&'a str>, mut x: HashSet<&'a str>, adjacent: &HashMap<&str, HashSet<&'a str>>) -> HashSet<Vec<&'a str>> {
    if p.len() == 0 && x.len() == 0 {r.sort(); return HashSet::from([r])};
    let mut out = HashSet::new();
    while let Some(&v) = p.iter().next() {
        let mut r2 = r.clone(); r2.push(v);
        let n = adjacent.get(&v).unwrap();
        let p2 = p.intersection(n).copied().collect();
        let x2 = x.intersection(n).copied().collect();
        out.extend(max_clique(r2, p2, x2, adjacent));
        p.remove(v);
        x.insert(v);
    }
    out
}

fn part2(inp: &str) -> impl Display {
    let graph = inp.lines().map(|l| {
        let (a, b) = l.split('-').collect_tuple().unwrap();
        [(a, b), (b, a)]
    }).flatten().collect::<HashSet<(&str, &str)>>();
    let mut adjacent: HashMap<&str, HashSet<&str>> = HashMap::new();
    for g in &graph {
        if let Some(a) = adjacent.get_mut(&g.0) {
            a.insert(g.1);
        } else {
            adjacent.insert(g.0, HashSet::from([g.1]));
        }
    }

    let mut best = 0;
    let mut best_clique = vec![];
    for c in max_clique(
        vec![],
        adjacent.keys().copied().collect::<HashSet<_>>(),
        HashSet::new(),
        &adjacent) {
            if c.len() > best {
                best = c.len();
                best_clique = c;
            }
    }

    best_clique.into_iter().sorted().join(",")
}

fn main() {
    let inp = load_input("day23");
    println!("{}", part2(&inp));
}