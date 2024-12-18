fn main() {
    let input = include_str!("./input6.txt");

    let input_lines = input.lines();

    let mut grid: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    let number_of_lines: i32 = grid.len() as i32;
    let chars_per_line: i32 = grid[0].len() as i32;

    let mut count = 1;
    let mut pos_x = 0;
    let mut pos_y = 0;

    for x in 0..number_of_lines {
        for y in 0..chars_per_line {
            let curr = get_char(&grid, x, y);
            if curr == '^' || curr == 'V' || curr == '>'|| curr == '<' {
                pos_x = x;
                pos_y = y;
                break;
            }
        }
    }

    loop {
        let curr = get_char(&grid, pos_x, pos_y);

        if curr == 'X' {
            break;
        }
        else if curr == '^' {
            let above = get_char(&grid, pos_x - 1, pos_y);
            if above == '#' {
                count += set_char(&mut grid, pos_x, pos_y+1,  '>');
                pos_y = pos_y + 1;
            }
            else {
                count += set_char(&mut grid, pos_x - 1, pos_y, '^');
                pos_x = pos_x - 1;
            }
        }
        else if curr == 'V' {
            let below = get_char(&grid, pos_x + 1, pos_y);
            if below == '#' {
                count += set_char(&mut grid, pos_x, pos_y-1,  '<');
                pos_y = pos_y - 1;
            }
            else {
                count += set_char(&mut grid, pos_x + 1, pos_y, 'V');
                pos_x = pos_x + 1;
            }
        }
        else if curr == '>' {
            let right = get_char(&grid, pos_x, pos_y + 1);
            if right == '#' {
                count += set_char(&mut grid, pos_x + 1, pos_y, 'V');
                pos_x = pos_x + 1;
            }
            else {
                count += set_char(&mut grid, pos_x, pos_y + 1, '>');
                pos_y = pos_y + 1;
            }
        }
        else if curr == '<' {
            let left = get_char(&grid, pos_x, pos_y - 1);
            if left == '#' {
                count += set_char(&mut grid, pos_x - 1, pos_y, '^');
                pos_x = pos_x - 1;
            }
            else {
                count += set_char(&mut grid, pos_x, pos_y - 1, '<');
                pos_y = pos_y - 1;
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

fn set_char(grid: &mut Vec<Vec<char>>, x: i32, y: i32, value: char) -> i32 {
    if x < 0 {
        return 0;
    }
    if x >= grid.len() as i32 {
        return 0;
    }
    if y < 0 {
        return 0;
    }
    if y >= grid[0].len() as i32 {
        return 0;
    }

    let existing = grid[x as usize][y as usize];

    grid[x as usize][y as usize] = value;

    return if existing == '.' { 1 } else  { 0 };
}