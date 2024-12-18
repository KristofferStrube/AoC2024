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

        let three: i32 = 3;

        for x in 0..three.pow(operators_length.try_into().unwrap()) {
            let mut operators: Vec<char> = Vec::new();
            
            for i in 0..operators_length {
                if (x as i32 / three.pow(i.try_into().unwrap())) % 3 == 1 {
                    operators.push('+');
                }
                else if (x as i32 / three.pow(i.try_into().unwrap())) % 3 == 2 {
                    operators.push('*');
                }
                else {
                    operators.push('|');
                }
            }

            let mut calculation: i64 = numbers[0];

            for i in 1..num_len {
                match operators[i-1] {
                    '+' => {
                        calculation += numbers[i];
                    },
                    '*' => {
                        calculation *= numbers[i];
                    }
                    _ => {
                        let mut calculation_string = calculation.to_string();
                        calculation_string.push_str(&numbers[i].to_string());
                        calculation = calculation_string.parse().unwrap()
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
