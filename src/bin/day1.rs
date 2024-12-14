#![feature(iter_advance_by, map_try_insert, portable_simd)]
#![cfg_attr(test, feature(test))]
#![allow(static_mut_refs)]

#[cfg(test)]
extern crate test;

use std::{collections::HashMap, hash::BuildHasherDefault, hint::assert_unchecked};

use aoc24::load_input;
use identity_hash::IdentityHasher;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

static mut N1S: [u32; 1000] = [0u32; 1000];
static mut N2S: [u32; 1000] = [0u32; 1000];

#[allow(unused)]
fn part1(inp: &[u8]) -> u32 {
    let mut j = 0usize;

    for i in 0..1000 {
        let l = unsafe {inp.get_unchecked(j..=j.unchecked_add(12))};
        let n1 = unsafe {
              *l.get_unchecked(0) as u32*10000
            + *l.get_unchecked(1) as u32*1000
            + *l.get_unchecked(2) as u32*100
            + *l.get_unchecked(3) as u32*10
            + *l.get_unchecked(4) as u32
        };
        
        let n2 = unsafe {
              *l.get_unchecked(8)  as u32*10000
            + *l.get_unchecked(9)  as u32*1000
            + *l.get_unchecked(10) as u32*100
            + *l.get_unchecked(11) as u32*10
            + *l.get_unchecked(12) as u32
        };

        unsafe {assert_unchecked(i < 1000);}
        unsafe {N1S[i] = n1;}
        unsafe {N2S[i] = n2;}
        
        j = unsafe {j.unchecked_add(14)};
    }

    unsafe { N1S.sort_unstable(); }
    unsafe { N2S.sort_unstable(); }

    unsafe {(0..1000).map(|i| N1S[i].abs_diff(N2S[i])).fold(0, |a, b| a.unchecked_add(b))}
}

#[allow(unused)]
fn part2(inp: &[u8]) -> u32 {
    let mut j = 0usize;
    
    let mut n2s: HashMap<u32, u32, BuildHasherDefault<IdentityHasher<u32>>>
        = HashMap::with_capacity_and_hasher(1000, BuildHasherDefault::default());

    for i in 0..1000 {
        let l = unsafe {inp.get_unchecked(j..=j.unchecked_add(12))};
        let n1 = unsafe {
            *l.get_unchecked(0) as u32*10000
            + *l.get_unchecked(1) as u32*1000
            + *l.get_unchecked(2) as u32*100
            + *l.get_unchecked(3) as u32*10
            + *l.get_unchecked(4) as u32
        } - const {'0' as u8 as u32 * 11111};
        
        let n2 = unsafe {
            *l.get_unchecked(8)  as u32*10000
            + *l.get_unchecked(9)  as u32*1000
            + *l.get_unchecked(10) as u32*100
            + *l.get_unchecked(11) as u32*10
            + *l.get_unchecked(12) as u32
        } - const {'0' as u8 as u32 * 11111};

        unsafe {assert_unchecked(i < 1000);}
        unsafe {N1S[i] = n1};
        if n2s.try_insert(n2, 1).is_err() {
            unsafe {*n2s.get_mut(&n2).unwrap_unchecked() += 1}
        }
        
        j = unsafe {j.unchecked_add(14)};
    }
    // println!("{:?}", n2s);
    unsafe {N1S.iter().map(|x| x*n2s.get(&x).copied().unwrap_or(0u32)).sum()}
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use aoc24::load_input;
    use test::Bencher;

    use crate::{part1, part2};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let inp = load_input("day1");
        b.iter(|| {
            black_box(part1(inp.as_bytes()));
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let inp = load_input("day1");
        b.iter(|| {
            black_box(part2(inp.as_bytes()));
        });
    }
}

fn main() {
    let inp = load_input("day1");
    println!("{}", part1(inp.as_bytes()));
    println!("{}", part2(inp.as_bytes()));
}
