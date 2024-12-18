use std::collections::HashMap;

fn main() {
    let input = include_str!("./input6.txt");

    let input_lines = input.lines();

    let mut grid: Vec<Vec<char>> = input_lines
        .clone()
        .map(|line| line.chars().collect())
        .collect();

    let number_of_lines: i32 = grid.len() as i32;
    let chars_per_line: i32 = grid[0].len() as i32;

    let mut count = 0;
    let mut origin_pos_x = 0;
    let mut origin_pos_y = 0;

    for x in 0..number_of_lines {
        for y in 0..chars_per_line {
            let curr = get_char(&grid, x, y);
            if curr == '^' || curr == 'V' || curr == '>' || curr == '<' {
                origin_pos_x = x;
                origin_pos_y = y;
                break;
            }
        }
    }

    for x in 0..number_of_lines {
        println!("{}", x);
        for y in 0..chars_per_line {
            grid = input_lines
                .clone()
                .map(|line| line.chars().collect())
                .collect();

            let curr = get_char(&grid, x, y);
            if curr == '#' || curr == 'X' || curr == '^' || curr == 'V' || curr == '>' || curr == '<' {
                continue;
            }
            set_char(&mut grid, x, y, '#');

            let mut pos_x = origin_pos_x;
            let mut pos_y = origin_pos_y;

            let mut visited: HashMap<(i32, i32), Vec<char>> = HashMap::new();

            loop {
                let curr = get_char(&grid, pos_x, pos_y);

                if track_visited(&mut visited, pos_x, pos_y, curr) {
                    count += 1;
                    break;
                }
                if curr == 'X' {
                    break;
                } else if curr == '^' {
                    let above = get_char(&grid, pos_x - 1, pos_y);
                    if above == '#' {
                        set_char(&mut grid, pos_x, pos_y, '>');
                    } else {
                        set_char(&mut grid, pos_x - 1, pos_y, '^');
                        pos_x = pos_x - 1;
                    }
                } else if curr == 'V' {
                    let below = get_char(&grid, pos_x + 1, pos_y);
                    if below == '#' {
                        set_char(&mut grid, pos_x, pos_y, '<');
                    } else {
                        set_char(&mut grid, pos_x + 1, pos_y, 'V');
                        pos_x = pos_x + 1;
                    }
                } else if curr == '>' {
                    let right = get_char(&grid, pos_x, pos_y + 1);
                    if right == '#' {
                        set_char(&mut grid, pos_x, pos_y, 'V');
                    } else {
                        set_char(&mut grid, pos_x, pos_y + 1, '>');
                        pos_y = pos_y + 1;
                    }
                } else if curr == '<' {
                    let left = get_char(&grid, pos_x, pos_y - 1);
                    if left == '#' {
                        set_char(&mut grid, pos_x, pos_y, '^');
                    } else {
                        set_char(&mut grid, pos_x, pos_y - 1, '<');
                        pos_y = pos_y - 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn get_char(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 {
        return 'X';
    }
    if x >= grid.len() as i32 {
        return 'X';
    }
    if y < 0 {
        return 'X';
    }
    if y >= grid[0].len() as i32 {
        return 'X';
    }

    return grid[x as usize][y as usize];
}

fn set_char(grid: &mut Vec<Vec<char>>, x: i32, y: i32, value: char) -> bool {
    if x < 0 {
        return true;
    }
    if x >= grid.len() as i32 {
        return true;
    }
    if y < 0 {
        return true;
    }
    if y >= grid[0].len() as i32 {
        return true;
    }

    let existing = grid[x as usize][y as usize];

    grid[x as usize][y as usize] = value;

    return existing != value;
}

fn track_visited(visited: &mut HashMap<(i32, i32), Vec<char>>, x: i32, y: i32, value: char) -> bool {
    let entry = visited.get_mut(&(x, y));

    match entry {
        None => {
            let mut chars = Vec::new();
            chars.push(value);
            visited.insert((x, y), chars);
            return false;
        },
        Some(chars) => {
            if chars.contains(&value) {
                return true;
            }
            chars.push(value);
            return false;
        }
    }
}