fn main() {
    let input = include_str!("./input1.txt");

    let input_lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for part in input_lines {
        let line_parts: Vec<&str> = part.split("   ").collect();
        left.push(line_parts[0].trim().parse().unwrap());
        right.push(line_parts[1].trim().parse().unwrap());
    }

    left.sort();
    right.sort();

    let sum_of_lines: i32 = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();

    println!("{}", sum_of_lines);
}
    