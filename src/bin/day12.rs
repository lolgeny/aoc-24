use aoc24::load_input;
use itertools::Itertools;
use partitions::PartitionVec;

fn part1(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut groups = PartitionVec::new();
    let x_len = grid[0].len(); let y_len = grid.len();
    for y in 0..y_len {
        for x in 0..x_len {
            groups.push((x,y));
            let c = grid[y][x];
            if x > 0 && grid[y][x-1] == c {groups.union(y*x_len+x, y*x_len+x-1);}
            if y > 0 && grid[y-1][x] == c {groups.union(y*x_len+x, (y-1)*x_len+x);}
        }
    }
    
    let mut total = 0;
    for g in groups.all_sets() {
        let mut area = 0;
        let mut corners = 0;
        for (_, &(x, y)) in g {
            let c = grid[y][x];
            let x = x as isize; let y = y as isize;
            area += 1;
            let (e1, e2, e3, e4, e5, e6, e7, e8) =
                [(x-1,y),(x,y-1),(x+1,y),(x,y+1), (x+1,y+1),(x+1,y-1),(x-1,y-1),(x-1,y+1)]
                .into_iter()
                .map(|(x2, y2)| if x2 >= 0 && y2 >= 0 && x2 < x_len as isize && y2 < y_len as isize {
                    grid[y2 as usize][x2 as usize] != c
                } else {true})
                .collect_tuple().unwrap();
            corners += [(e1, e2, e7), (e2, e3, e6), (e3, e4, e5), (e4, e1, e8)].into_iter()
                .filter(|&(a, b, c)| (a && b) || (!a && !b && c))
                .count();
        }
        total += area*corners;
    }

    total
}

fn main() {
    let inp = load_input("day12");
    println!("{}", part1(&inp));
}