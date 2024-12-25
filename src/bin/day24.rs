#![feature(let_chains)]

use std::{array, fmt::Display};

use aoc24::load_input;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Wire<'a> {
    Const(bool), Op(&'a str, &'a str, WireOp)
}
impl<'a> Wire<'a> {
    pub fn get(self) -> Option<bool> {
        match self {
            Wire::Const(b) => Some(b),
            Wire::Op(..) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum WireOp {
    And, Or, Xor
}

fn parse_input(inp: &str) -> HashMap<&str, Wire> {
    let (a, b) = inp.split("\n\n").collect_tuple().unwrap();
    let mut wires: HashMap<&str, Wire> = a.lines().map(|l| {
        let (k, v) = l.split(": ").collect_tuple().unwrap();
        (k, match v {"1" => Wire::Const(true), "0" => Wire::Const(false), _ => panic!()})
    }).collect();

    for l in b.lines() {
        let (i, op, j, _, k) = l.split(" ").collect_tuple().unwrap();
        wires.insert(k, match op {
            "AND" => Wire::Op(i, j, WireOp::And),
            "OR" => Wire::Op(i, j, WireOp::Or),
            "XOR" => Wire::Op(i, j, WireOp::Xor),
            _ => panic!()
        });
    }
    wires
}

fn simulate<'a>(mut wires: HashMap<&'a str, Wire<'a>>) -> HashMap<&'a str, Wire<'a>> {
    let wire_keys = wires.keys().copied().collect_vec();

    loop {
        let mut finished = true;
        for &k in &wire_keys {
            let (i, j, op) = match wires.get(&k).unwrap() {
                Wire::Const(_) => {continue},
                Wire::Op(i, j, op) => {(*i, *j, *op)}
            };
            finished = false;
            if let Some(x) = wires.get(&i).unwrap().get()
                && let Some(y) = wires.get(&j).unwrap().get() {
                    wires.insert(k, match op {
                        WireOp::And => Wire::Const(x && y),
                        WireOp::Or => Wire::Const(x || y),
                        WireOp::Xor => Wire::Const(x ^ y),
                    });
            }
        }
        if finished {break};
    }
    wires
}

fn simulate2<'a>(mut wires: HashMap<&'a str, Wire<'a>>, init: &HashSet<&'a str>, dependents: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Wire<'a>> {
    let mut queue = Vec::from_iter(init.iter().copied());
    let mut visited = HashSet::new();
    while let Some(n) = queue.pop() {
        if visited.contains(&n) {continue};
        queue.extend(dependents.get(n).unwrap());

        let (i, j, op) = match wires.get(&n).unwrap() {
            Wire::Const(_) => {visited.insert(n); continue},
            Wire::Op(i, j, op) => {(*i, *j, *op)}
        };
        if let Some(x) = wires.get(&i).unwrap().get()
            && let Some(y) = wires.get(&j).unwrap().get() {
                wires.insert(n, match op {
                    WireOp::And => Wire::Const(x && y),
                    WireOp::Or => Wire::Const(x || y),
                    WireOp::Xor => Wire::Const(x ^ y),
                });
                visited.insert(n);
        }
    }

    wires
}

struct WireExtractor {
    keys: Vec<String>
}
impl WireExtractor {
    pub fn new(wires: &HashMap<&str, Wire>, prefix: char) -> Self {
        let mut keys = vec![];
        let mut i = 0;
        loop {
            let k = format!("{prefix}{:02}", i);
            if let Some(_) = wires.get(k.as_str()) {
                keys.push(k);
            } else {break}
            i += 1;
        }
        Self {keys}
    }
    pub fn extract(&self, wires: &HashMap<&str, Wire>) -> u64 {
        let mut n = 0;
        for i in 0..self.keys.len() {
            n += (wires.get(self.keys[i].as_str()).unwrap().get().unwrap() as u64) << i;
        }
        n
    }
    pub fn bits(&self) -> usize {
        self.keys.len()
    }
    pub fn set<'a, 'b: 'a>(&'b self, wires: &mut HashMap<&'a str, Wire>, mut n: u64) {
        for i in 0..self.bits() {
            wires.insert(self.keys[i].as_str(), Wire::Const(n & 1 == 1));
            n >>= 1;
        }
    }
    pub fn get_key(&self, i: usize) -> &str {
        self.keys[i].as_str()
    }
}

fn get_bit(n: u64, b: u64) -> bool {
    n & (1 << b) != 0
}

#[allow(unused)]
fn part1(inp: &str) -> impl Display {
    let wires = simulate(parse_input(inp));

    let extractor = WireExtractor::new(&wires, 'z');
    extractor.extract(&wires)
}

fn swap<'a>(wires: &mut HashMap<&'a str, Wire<'a>>, i: &'a str, j: &'a str) {
    let a = *wires.get(i).unwrap();
    let b = *wires.get(j).unwrap();
    wires.insert(i, b);
    wires.insert(j, a);
}

#[allow(unused)]
fn part2(inp: &str) -> impl Display {
    let wires: HashMap<&str, Wire<'_>> = parse_input(inp);
    
    let x_extract = WireExtractor::new(&wires, 'x');
    let y_extract = WireExtractor::new(&wires, 'y');

    let init: HashSet<_> = (0..x_extract.bits()).map(|b| x_extract.get_key(b))
        .chain((0..y_extract.bits()).map(|b| y_extract.get_key(b)))
        .collect();

    let z_extract = WireExtractor::new(&wires, 'z');

    let mut dependencies = HashMap::new();
    let mut fwd_dependencies = HashMap::new();
    let mut dependents = HashMap::new();
    for &k in wires.keys() {
        fwd_dependencies.insert(k, HashSet::new());
        dependents.insert(k, vec![]);

    }
    for &k in wires.keys() {
        let mut deps = HashSet::new();
        let mut queue = vec![k];
        let mut first = true;
        while let Some(n) = queue.pop() {
            if deps.contains(&n) {continue};
            deps.insert(n);
            fwd_dependencies.get_mut(&n).unwrap().insert(k);
            if let &Wire::Op(i, j, _) = wires.get(n).unwrap() {
                queue.push(i); queue.push(j);
                if first {
                    dependents.get_mut(&i).unwrap().push(k);
                    dependents.get_mut(&j).unwrap().push(k);
                }
            }
            first = false;
        }
        deps.remove(k);
        dependencies.insert(k, deps);
    }

    // swap(&mut wires, "nng", "psw");

    let mut broken = HashSet::new();
    let mut rng = thread_rng();
    let mut wires2 = wires.clone();
    for _ in 0..1000 {
        let x = rng.gen_range(0..1<<x_extract.bits() as u64);
        x_extract.set(&mut wires2, x);
        let y = rng.gen_range(0..1<<y_extract.bits() as u64);
        y_extract.set(&mut wires2, y);
        let expected = x + y;

        let out = simulate(wires.clone());
        let z = z_extract.extract(&out);

        for i in 0..z_extract.bits() {
            if get_bit(z, i as u64) != get_bit(expected, i as u64) {
                broken.insert(i);
            }
        }
    }
    println!("{:?}", broken.iter().sorted());
    let mut faulty: HashSet<&str> = HashSet::new();
    let mut not_faulty = HashSet::new();
    for b in 0..z_extract.bits() {
        let it = dependencies.get(&z_extract.get_key(b)).unwrap().iter().copied();
        if broken.contains(&b) {
            faulty.extend(it); faulty.insert(z_extract.get_key(b));
        } else {
            not_faulty.extend(it);
        }
    }
    faulty.retain(|x| !not_faulty.contains(x));
    let all_faulty = faulty.clone();
    let mut io_mask = HashSet::new();
    for a in 0..x_extract.bits() {
        io_mask.insert(x_extract.get_key(a));
    }
    for a in 0..y_extract.bits() {
        io_mask.insert(y_extract.get_key(a));
    }
    for a in 0..z_extract.bits() {
        io_mask.insert(z_extract.get_key(a));
    }
    
    faulty.retain(|x| !io_mask.contains(x));

    // for x in faulty {
    //     println!("{x}: {:?}", dependencies.get(&x).unwrap());
    // }

    let mut root_faulty = HashSet::new();
    for &x in &faulty {
        let mut queue = vec![x];
        let mut visited = HashSet::new();
        while let Some(n) = queue.pop() {
            if visited.contains(&n) {continue};
            visited.insert(n);
            if let Wire::Op(i, j, _) = wires.get(n).unwrap() {
                let a = faulty.contains(i);
                let b = faulty.contains(j);
                if a { queue.push(*i); }
                if b { queue.push(*j); }
                if !(a || b) { root_faulty.insert(n); }
            } else { panic!("Should not have const wire as faulty") };
        }
    }
    println!("{:?}", root_faulty.iter().sorted());
    println!("{:?}", root_faulty.len());

    // it's trees all the way down
    // let mut to_blame = HashSet::new();
    // for &x in &root_faulty {
    //     let mut queue = vec![x];
    //     let mut visited = HashSet::new();
    //     while let Some(n) = queue.pop() {
    //         if visited.contains(&n) {continue};
    //         visited.insert(n);
    //         if let Wire::Op(i, j, _) = wires.get(n).unwrap() {
    //             let a = fwd_dependencies.get(i).unwrap().is_subset(&all_faulty);
    //             let b = fwd_dependencies.get(j).unwrap().is_subset(&all_faulty);
    //             if a { to_blame.insert(*i); }
    //             if b { to_blame.insert(*j); }
    //             queue.push(i); queue.push(j);
    //         }
    //     }
    // }
    // to_blame.retain(|x| !io_mask.contains(x));
    // println!("{:?}", to_blame.iter().sorted());
    // println!("{:?}", to_blame.len());

    let out = simulate2(wires.clone(), &init, &dependents);
    println!("simulating...");
    println!("{}", z_extract.extract(&out));

    let x_range = 0..1<<x_extract.bits() as u64;
    let y_range = 0..1<<y_extract.bits() as u64;
    let xs: [u64; 5] = array::from_fn(|_| rng.gen_range(x_range.clone()));
    let ys: [u64; 5] = array::from_fn(|_| rng.gen_range(y_range.clone()));
    let zs: [u64; 5] = array::from_fn(|i| xs[i] + ys[i]);
    for p in root_faulty.iter().combinations(8) {
        'perms: for (a, b, c, d, e, f, g, h) in COMBINATIONS {
            let mut wires2 = wires.clone();
            let (a, b, c, d, e, f, g, h) = (p[a], p[b], p[c], p[d], p[e], p[f], p[g], p[h]);
            swap(&mut wires2, a, b);
            swap(&mut wires2, c, d);
            swap(&mut wires2, e, f);
            swap(&mut wires2, g, h);

            for i in 0..5 {
                x_extract.set(&mut wires2, xs[i]);
                y_extract.set(&mut wires2, ys[i]);
        
                let out = simulate2(wires.clone(), &init, &dependents);
                let z = z_extract.extract(&out);
        
                if z != zs[i] {continue 'perms};
            }
            println!("FOUND IT!!!!!!! {a}, {b}, {c}, {d}, {e}, {f}, {g}, {h}");
        }
    }
    0
}


fn part2_2(inp: &str) -> impl Display {
    let mut wires: HashMap<&str, Wire<'_>> = parse_input(inp);

// gsd,kth,qnf,tbt,vpm,z12,z26,z32
    swap(&mut wires, "vpm", "qnf");
    swap(&mut wires, "z12", "kth"); // yep
    swap(&mut wires, "gsd", "z26"); // yep
    swap(&mut wires, "z32", "tbt"); // ?
    
    let x_extract = WireExtractor::new(&wires, 'x');
    let y_extract = WireExtractor::new(&wires, 'y');
    let z_extract = WireExtractor::new(&wires, 'z');

    let in_bits = x_extract.bits();
    assert_eq!(in_bits, y_extract.bits());

    for b in 1..(in_bits-1) {
        println!("{b}");
        let xb = x_extract.get_key(b);
        let yb = y_extract.get_key(b);

        let xor_i = wires.iter().filter(
            |(_, v)| matches!(v, Wire::Op(i, j, WireOp::Xor) if (*i == xb && *j == yb) || (*i == yb && *j == xb))
        ).next().unwrap().0;

        // and check
        let xor2_i = wires.iter().filter(|(_, v)|
            matches!(v, Wire::Op(i, j, WireOp::Xor) if i == xor_i || j == xor_i)
        ).next().expect(&format!("Could not find, b={b}, xor_i={xor_i}")).0;
        if *xor2_i != z_extract.get_key(b) {
            println!("BAD! {xor_i} points to {xor2_i}");
        }

        // xor check
    }

    let mut rng = thread_rng();
    let mut wires2 = wires.clone();
    let x = rng.gen_range(0..(1 << in_bits));
    let y = rng.gen_range(0..(1 << in_bits));
    x_extract.set(&mut wires2, x);
    y_extract.set(&mut wires2, y);
    let out = simulate(wires2);
    let n = z_extract.extract(&out);
    let x = x_extract.extract(&out);
    let y = y_extract.extract(&out);
    println!("{x}+{y}={}; got {n} -- abs diff {}", x+y, (x+y).abs_diff(n));
    0
}


fn main() {
    let inp = load_input("day24");
    println!("{}", part2_2(&inp));
}

static COMBINATIONS: [(usize, usize, usize, usize, usize, usize, usize, usize); 105] = [(0, 1, 2, 3, 4, 5, 6, 7), (0, 1, 2, 3, 4, 6, 5, 7), (0, 1, 2, 3, 4, 7, 5, 6), (0, 1, 2, 4, 3, 5, 6, 7), (0, 1, 2, 4, 3, 6, 5, 7), (0, 1, 2, 4, 3, 7, 5, 6), (0, 1, 2, 5, 3, 4, 6, 7), (0, 1, 2, 5, 3, 6, 4, 7), (0, 1, 2, 5, 3, 7, 4, 6), (0, 1, 2, 6, 3, 4, 5, 7), (0, 1, 2, 6, 3, 5, 4, 7), (0, 1, 2, 6, 3, 7, 4, 5), (0, 1, 2, 7, 3, 4, 5, 6), (0, 1, 2, 7, 3, 5, 4, 6), (0, 1, 2, 7, 3, 6, 4, 5), (0, 2, 1, 3, 4, 5, 6, 7), (0, 2, 1, 3, 4, 6, 5, 7), (0, 2, 1, 3, 4, 7, 5, 6), (0, 2, 1, 4, 3, 5, 6, 7), (0, 2, 1, 4, 3, 6, 5, 7), (0, 2, 1, 4, 3, 7, 5, 6), (0, 2, 1, 5, 3, 4, 6, 7), (0, 2, 1, 5, 3, 6, 4, 7), (0, 2, 1, 5, 3, 7, 4, 6), (0, 2, 1, 6, 3, 4, 5, 7), (0, 2, 1, 6, 3, 5, 4, 7), (0, 2, 1, 6, 3, 7, 4, 5), (0, 2, 1, 7, 3, 4, 5, 6), (0, 2, 1, 7, 3, 5, 4, 6), (0, 2, 1, 7, 3, 6, 4, 5), (0, 3, 1, 2, 4, 5, 6, 7), (0, 3, 1, 2, 4, 6, 5, 7), (0, 3, 1, 2, 4, 7, 5, 6), (0, 3, 1, 4, 2, 5, 6, 7), (0, 3, 1, 4, 2, 6, 5, 7), (0, 3, 1, 4, 2, 7, 5, 6), (0, 3, 1, 5, 2, 4, 6, 7), (0, 3, 1, 5, 2, 6, 4, 7), (0, 3, 1, 5, 2, 7, 4, 6), (0, 3, 1, 6, 2, 4, 5, 7), (0, 3, 1, 6, 2, 5, 4, 7), (0, 3, 1, 6, 2, 7, 4, 5), (0, 3, 1, 7, 2, 4, 5, 6), (0, 3, 1, 7, 2, 5, 4, 6), (0, 3, 1, 7, 2, 6, 4, 5), (0, 4, 1, 2, 3, 5, 6, 7), (0, 4, 1, 2, 3, 6, 5, 7), (0, 4, 1, 2, 3, 7, 5, 6), (0, 4, 1, 3, 2, 5, 6, 7), (0, 4, 1, 3, 2, 6, 5, 7), (0, 4, 1, 3, 2, 7, 5, 6), (0, 4, 1, 5, 2, 3, 6, 7), (0, 4, 1, 5, 2, 6, 3, 7), (0, 4, 1, 5, 2, 7, 3, 6), (0, 4, 1, 6, 2, 3, 5, 7), (0, 4, 1, 6, 2, 5, 3, 7), (0, 4, 1, 6, 2, 7, 3, 5), (0, 4, 1, 7, 2, 3, 5, 6), (0, 4, 1, 7, 2, 5, 3, 6), (0, 4, 1, 7, 2, 6, 3, 5), (0, 5, 1, 2, 3, 4, 6, 7), (0, 5, 1, 2, 3, 6, 4, 7), (0, 5, 1, 2, 3, 7, 4, 6), (0, 5, 1, 3, 2, 4, 6, 7), (0, 5, 1, 3, 2, 6, 4, 7), (0, 5, 1, 3, 2, 7, 4, 6), (0, 5, 1, 4, 2, 3, 6, 7), (0, 5, 1, 4, 2, 6, 3, 7), (0, 5, 1, 4, 2, 7, 3, 6), (0, 5, 1, 6, 2, 3, 4, 7), (0, 5, 1, 6, 2, 4, 3, 7), (0, 5, 1, 6, 2, 7, 3, 4), (0, 5, 1, 7, 2, 3, 4, 6), (0, 5, 1, 7, 2, 4, 3, 6), (0, 5, 1, 7, 2, 6, 3, 4), (0, 6, 1, 2, 3, 4, 5, 7), (0, 6, 1, 2, 3, 5, 4, 7), (0, 6, 1, 2, 3, 7, 4, 5), (0, 6, 1, 3, 2, 4, 5, 7), (0, 6, 1, 3, 2, 5, 4, 7), (0, 6, 1, 3, 2, 7, 4, 5), (0, 6, 1, 4, 2, 3, 5, 7), (0, 6, 1, 4, 2, 5, 3, 7), (0, 6, 1, 4, 2, 7, 3, 5), (0, 6, 1, 5, 2, 3, 4, 7), (0, 6, 1, 5, 2, 4, 3, 7), (0, 6, 1, 5, 2, 7, 3, 4), (0, 6, 1, 7, 2, 3, 4, 5), (0, 6, 1, 7, 2, 4, 3, 5), (0, 6, 1, 7, 2, 5, 3, 4), (0, 7, 1, 2, 3, 4, 5, 6), (0, 7, 1, 2, 3, 5, 4, 6), (0, 7, 1, 2, 3, 6, 4, 5), (0, 7, 1, 3, 2, 4, 5, 6), (0, 7, 1, 3, 2, 5, 4, 6), (0, 7, 1, 3, 2, 6, 4, 5), (0, 7, 1, 4, 2, 3, 5, 6), (0, 7, 1, 4, 2, 5, 3, 6), (0, 7, 1, 4, 2, 6, 3, 5), (0, 7, 1, 5, 2, 3, 4, 6), (0, 7, 1, 5, 2, 4, 3, 6), (0, 7, 1, 5, 2, 6, 3, 4), (0, 7, 1, 6, 2, 3, 4, 5), (0, 7, 1, 6, 2, 4, 3, 5), (0, 7, 1, 6, 2, 5, 3, 4)];