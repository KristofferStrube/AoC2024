use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("./input10.txt");

    let input_lines = input.lines();

    let grid: Vec<Vec<u32>> = input_lines.map(|line| line.chars().map(|n|n.to_digit(10).unwrap()).collect()).collect();

    let number_of_lines: i32 = grid.len() as i32;
    let chars_per_line: i32 = grid[0].len() as i32;

    let mut trailheads: Vec<(i32, i32)> = Vec::new();

    for x in 0..number_of_lines {
        for y in 0..chars_per_line {
            let curr = get_char(&grid, x, y);
            if curr == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let mut sum = 0;

    for trailhead in trailheads {
        let mut queue = VecDeque::new();
        let mut trailends = HashSet::new();
        queue.push_back((trailhead.0, trailhead.1, 0));
        while queue.len() > 0 {
            let entry = queue.pop_front().unwrap();
            if entry.2 == 9 {
                trailends.insert((entry.0, entry.1));
                continue;
            }

            let up = get_char(&grid, entry.0 - 1, entry.1);
            let down = get_char(&grid, entry.0 + 1, entry.1);
            let left = get_char(&grid, entry.0, entry.1 - 1);
            let right = get_char(&grid, entry.0, entry.1 + 1);
            if up == entry.2 + 1 {
                queue.push_back((entry.0 - 1, entry.1, entry.2 + 1));
            }
            if down == entry.2 + 1 {
                queue.push_back((entry.0 + 1, entry.1, entry.2 + 1));
            }
            if left == entry.2 + 1 {
                queue.push_back((entry.0, entry.1 - 1, entry.2 + 1));
            }
            if right == entry.2 + 1 {
                queue.push_back((entry.0, entry.1 + 1, entry.2 + 1));
            }
        }
        sum += trailends.len();
    }

    println!("{}", sum);
}

fn get_char(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> u32 {
    if x < 0 {
        return 100;
    }
    if x >= grid.len() as i32 {
        return 100;
    }
    if y < 0 {
        return 100;
    }
    if y >= grid[0].len() as i32 {
        return 100;
    }

    return grid[x as usize][y as usize];
}
