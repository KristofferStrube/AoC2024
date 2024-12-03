fn main() {
    let input: Vec<char> = include_str!("./input3.txt").chars().collect();

    let mut index = 0;

    let number_symbols = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut sum: i32 = 0;

    while index < input.len() {
        if input[index] != 'm' {
            index += 1;
            continue;
        }
        if input[index + 1] != 'u' {
            index += 1;
            continue;
        }
        if input[index + 2] != 'l' {
            index += 1;
            continue;
        }
        if input[index + 3] != '(' {
            index += 1;
            continue;
        }
        let mut first_number_sequence = 0;
        while number_symbols.contains(&input[index+4+first_number_sequence]) {
            first_number_sequence += 1;
        }
        if first_number_sequence == 0 {
            index += 1;
            continue;
        }
        if input[index+4+first_number_sequence] != ',' {
            index += 1;
            continue;
        }
        let mut second_number_sequence = 0;
        while number_symbols.contains(&input[index+4+first_number_sequence+1+second_number_sequence]) {
            second_number_sequence += 1;
        }
        if second_number_sequence == 0 {
            index += 1;
            continue;
        }
        if input[index+4+first_number_sequence+1+second_number_sequence] != ')' {
            index += 1;
            continue;
        }
        let left_string: String = input[index + 4..index+4+first_number_sequence].iter().collect();
        let left: i32 = left_string.parse().unwrap();
        let right_string: String = input[index+4+first_number_sequence+1..index+4+first_number_sequence+1+second_number_sequence].iter().collect();
        let rigth: i32 = right_string.parse().unwrap();

        sum += left * rigth;
        index += 8;
    }

    println!("{}", sum);
}
    