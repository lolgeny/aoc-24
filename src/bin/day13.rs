use aoc24::load_input;
use good_lp::{coin_cbc, constraint::eq, variables, Solution, SolverModel};
use itertools::Itertools;

fn part1(inp: &str) -> u64 {
    inp.split("\n\n").map(|chunk| {    variables! {
        vars:
            0 <= an (integer);// <= 100;
            0 <= bn (integer);// <= 100;
        }
        let ((ax,ay), (bx,by), (cx,cy)) = chunk.lines()
            .map(|l| l.split([' ', '+', ',', '='])
            .filter_map(|x: &str| x.parse::<u64>().ok()).collect_tuple::<(u64,u64)>().unwrap())
        .collect_tuple().unwrap();
        
        let problem = vars.minimise(3*an + bn).using(coin_cbc)
            .with(eq(ax as f64*an + bx as f64*bn, cx as f64))
            .with(eq(ay as f64*an + by as f64*bn, cy as f64));
        match problem.solve().ok() {
            Some(x) => {
                let a = x.value(an).round() as u64;
                let b = x.value(bn).round() as u64;
                println!("OPTIMALALITY");
                println!("{} {}", x.eval(an), x.eval(bn));
                3*a+b}
            None => {0}
        }
    }).sum()
}

fn part2(inp: &str) -> i64 {
    inp.split("\n\n").map(|chunk| {
        let ((ax,ay), (bx,by), (mut cx,mut cy)) = chunk.lines()
            .map(|l| l.split([' ', '+', ',', '='])
            .filter_map(|x: &str| x.parse::<i64>().ok()).collect_tuple::<(i64,i64)>().unwrap())
        .collect_tuple().unwrap();
        cx += 10000000000000; cy += 10000000000000;

        let det = ax*by - ay*bx;
        if det == 0 {return 0};
        let an = (by*cx - bx*cy)/det;
        let bn = (ax*cy - ay*cx)/det;
        if an < 0 || bn < 0 {return 0};
        if ax*an+bx*bn != cx || ay*an+by*bn != cy {return 0};
        println!("{an}, {bn}, {}, {}", ax*an+bx*bn, cx);
        3*an + bn
    }).sum()
}

fn main() {
    let inp = load_input("day13");
    println!("{}", part2(&inp));
}// >60915848588338