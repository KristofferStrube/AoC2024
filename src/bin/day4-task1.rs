fn main() {
    let input = include_str!("./input4.txt");

    let input_lines = input.lines();

    let grid: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    let number_of_lines: i32 = grid.len() as i32;
    let chars_per_line: i32 = grid[0].len() as i32;

    let mut count = 0;

    for x in 0..number_of_lines {
        for y in 0..chars_per_line {
            if get_char(&grid, x, y) != 'X' {
                continue;
            }

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1]  {
                    if get_char(&grid, x + 1 * i, y + 1 * j) == 'M'
                        && get_char(&grid, x + 2 * i, y + 2 * j) == 'A'
                        && get_char(&grid, x + 3 * i, y + 3 * j) == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn get_char(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 {
        return '.';
    }
    if x >= grid.len() as i32 {
        return '.';
    }
    if y < 0 {
        return '.';
    }
    if y >= grid[0].len() as i32 {
        return '.';
    }

    return grid[x as usize][y as usize];
}
