fn main() {
    let input = include_str!("./input2.txt");

    let input_lines = input.lines();

    let mut count: i32 = 0;
    for part in input_lines {
        let line_parts: Vec<i8> = part.split(" ").map(|n|n.parse().unwrap()).collect();

        let first = line_parts[0];
        let mut previous = line_parts[1];
        let mut diff = (previous - first).abs();
        if diff < 1 || diff > 3 {
            continue;
        }
        let increasing = previous > first;
        let mut success = true;
        for i in 2..line_parts.len() {
            if increasing != (line_parts[i] - previous > 0) {
                success = false;
                break;
            }

            diff = (line_parts[i] - previous).abs();
            if diff < 1 || diff > 3 {
                success = false;
                break;
            }
            previous = line_parts[i];
        }
        if success {
            count += 1;
        }
    }

    println!("{}", count);
}
    