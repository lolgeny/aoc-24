use aoc24::load_input;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len();
    let y_len = grid.len();
    assert_eq!(x_len, y_len);

    let mut flat = String::new();
    for r in &grid {flat.extend(r);
        flat.push('*');}
    for x in 0..x_len {
        flat.extend((0..y_len).map(|i| grid[i][x]));
        flat.push('-');
    }
    for i in 3..x_len {
        flat.extend((0..=i).map(|j| grid[j][i-j]));
        flat.push('|');
        flat.extend((0..=i).map(|j| grid[j][x_len-1-(i-j)]));
        flat.push('|');
        if i == x_len-1 {continue};
        flat.extend((0..=i).map(|j| grid[y_len-1-j][i-j]));
        flat.push('|');
        flat.extend((0..=i).map(|j| grid[y_len-1-j][x_len-1-(i-j)]));
        flat.push('|');
    }
    flat.matches("XMAS").count() + flat.matches("SAMX").count()
}

fn part2(inp: &str) -> usize {
    let grid = inp.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let x_len = grid[0].len();
    let y_len = grid.len();
    assert_eq!(x_len, y_len);
    let mut n = 0;

    for x in 1..(x_len-1) {
        for y in 1..(y_len-1) {
            if grid[y][x] != 'A' {continue}
            let mut c = 0;
            for (ax, ay) in [
                (1isize,1isize),(1,-1),(-1,1),(-1,-1)
            ] {
                if grid[(y as isize+ay) as usize][(x as isize+ax) as usize] == 'M'
                    && grid[(y as isize-ay) as usize][(x as isize-ax) as usize] == 'S' {
                        c += 1;
                    }
            }
            if c < 2 {continue};
            n += 1;
        }
    }

    n
}

fn main() {
    let inp = load_input("day4");
    println!("{}", part2(&inp));
}