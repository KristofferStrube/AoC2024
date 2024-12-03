fn main() {
    let input = include_str!("./input2.txt");

    let input_lines = input.lines();

    let mut count: i32 = 0;
    for part in input_lines {
        let line_parts: Vec<i8> = part.split(" ").map(|n|n.parse().unwrap()).collect();

        let mut success_in_any = false;
        for s in 0..line_parts.len() {
            let line_parts_skip_s: Vec<i8> = line_parts.iter().enumerate().filter(|&(i, _)| i != s).map(|(_, v)| *v).collect();
            
            let first = line_parts_skip_s[0];
            let mut previous = line_parts_skip_s[1];
            let mut diff = (previous - first).abs();
            if diff < 1 || diff > 3 {
                continue;
            }
            let increasing = previous > first;
            let mut success = true;
            for i in 2..line_parts_skip_s.len() {
                if increasing != (line_parts_skip_s[i] - previous > 0) {
                    success = false;
                    break;
                }

                diff = (line_parts_skip_s[i] - previous).abs();
                if diff < 1 || diff > 3 {
                    success = false;
                    break;
                }
                previous = line_parts_skip_s[i];
            }
            if success {
                success_in_any = true;
                break;
            }
        }
        if success_in_any {
            count += 1;
        }
    }

    println!("{}", count);
}
    