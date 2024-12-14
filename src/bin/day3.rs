use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) {
    let mut it = inp.chars();
    let mut total = 0;
    let mut enabled = true;
    while let Some(c) = it.next() {
        if c == 'm' {
            if !(it.next() == Some('u') && it.next() == Some('l') && it.next() == Some('(')) {continue}
            let i = match it.by_ref().peeking_take_while(|x| x.is_digit(10)).collect::<String>().parse::<u64>() {
                Ok(i) => i, Err(_) => continue
            };
            if i > 999 {continue}
            if it.next() != Some(',') {continue}
            let j = match it.by_ref().peeking_take_while(|x| x.is_digit(10)).collect::<String>().parse::<u64>() {
                Ok(i) => i, Err(_) => continue
            };
            if j > 999 {continue}
            if it.next() != Some(')') {continue}
            if enabled {total += i*j;}
        } else if c == 'd' {
            if it.next() != Some('o') {continue}
            match it.next() {
                Some('(') => {
                    if it.next() == Some(')') {enabled = true}
                }
                Some('n') => {
                    if it.next() == Some('\'') && it.next() == Some('t') && it.next() == Some('(') && it.next() == Some(')') {enabled = false};
                }
                _ => {}
            }
        }
    }
    println!("{total}");
}

fn main() {
    let inp = load_input("day3");
    part1(&inp);
}