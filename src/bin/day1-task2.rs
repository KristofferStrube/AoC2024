use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");

    let input_lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for part in input_lines {
        let line_parts: Vec<&str> = part.split("   ").collect();
        left.push(line_parts[0].trim().parse().unwrap());
        let count = right.entry(line_parts[1].trim().parse().unwrap()).or_insert(0);
        *count += 1;
    }

    let sum_of_lines: i32 = left.iter().map(|l| l * right.get(l).unwrap_or(&0)).sum();

    println!("{}", sum_of_lines);
}
    