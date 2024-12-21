use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input8.txt");

    let input_lines = input.lines();

    let grid: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    let number_of_lines: i32 = grid.len() as i32;
    let chars_per_line: i32 = grid[0].len() as i32;

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for x in 0..number_of_lines {
        for y in 0..chars_per_line {
            let curr = get_char(&grid, x, y);
            if curr == '.' {
                continue;
            }
            let entry = antennas.entry(curr).or_insert(Vec::new());
            entry.push((x, y));
        }
    }

    for group in antennas.values() {
        for first in group {
            for second in group {
                if first == second {
                    continue;
                }
                let positive_x = first.0 < second.0;
                let positive_y = first.1 < second.1;
                for i in 0..100 {
                    let first_antinode = (
                        if positive_x { first.0 - i * (second.0 - first.0) } else { first.0 + i * (first.0 - second.0) },
                        if positive_y { first.1 - i * (second.1 - first.1) } else { first.1 + i * (first.1 - second.1) }
                    );
                    if get_char(&grid, first_antinode.0, first_antinode.1) == '#' {
                        break;
                    }
                    antinodes.insert(first_antinode);
                }
                for i in 0..100 {
                    let second_antinode = (
                        if positive_x { second.0 + i * (second.0 - first.0) } else { second.0 - i * (first.0 - second.0) },
                        if positive_y { second.1 + i * (second.1 - first.1) } else { second.1 - i * (first.1 - second.1) }
                    );
                    if get_char(&grid, second_antinode.0, second_antinode.1) == '#' {
                        break;
                    }
                    antinodes.insert(second_antinode);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn get_char(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 {
        return '#';
    }
    if x >= grid.len() as i32 {
        return '#';
    }
    if y < 0 {
        return '#';
    }
    if y >= grid[0].len() as i32 {
        return '#';
    }

    return grid[x as usize][y as usize];
}
