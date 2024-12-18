fn main() {
    let input = include_str!("./input7.txt");

    let input_lines = input.lines();

    let mut sum = 0;

    for line in input_lines {
        let line_parts: Vec<&str> = line.split(": ").collect();

        let result: i64 = line_parts[0].parse().unwrap();

        let numbers: Vec<i64> = line_parts[1]
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        let num_len = numbers.len();
        let operators_length = num_len - 1;

        let two: i32 = 2;

        for x in 0..two.pow(operators_length.try_into().unwrap()) {
            let mut operators: Vec<char> = Vec::new();
            
            for i in 0..operators_length {
                if (x as i32 / two.pow(i.try_into().unwrap())) % 2 == 1 {
                    operators.push('+');
                }
                else {
                    operators.push('*');
                }
            }

            let mut calculation: i64 = numbers[0];

            for i in 1..num_len {
                match operators[i-1] {
                    '+' => {
                        calculation += numbers[i];
                    },
                    _ => {
                        calculation *= numbers[i];
                    }
                }
            }
            if result == calculation {
                sum += result;
                break;
            }
        }
    }

    println!("{}", sum);
}
