use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input5.txt");

    let input_lines = input.lines();

    let mut reading_rules = true;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut sum = 0;

    for line in input_lines {
        if line == "" {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let parts: Vec<i32> = line.split('|').map(|p| p.parse().unwrap()).collect();
            let rule = rules.entry(parts[1]).or_insert(Vec::new());
            rule.push(parts[0]);
        } else {
            let mut numbers: Vec<i32> = line.split(',').map(|p| p.parse().unwrap()).collect();
            let set: HashSet<i32> = HashSet::from_iter(numbers.iter().cloned());
            let mut seen_numbers: HashSet<i32> = HashSet::new();
            let mut valid = true;
            for i in numbers.iter() {
                let rule = rules.get(i);
                if rule.is_some() {
                    for prior in rule.unwrap() {
                        if set.contains(prior) && !seen_numbers.contains(prior) {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                seen_numbers.insert(*i);
            }
            if !valid {
                let mut new_numbers_order: Vec<i32> = Vec::new();
                let numbers_with_no_rules: Vec<i32> = numbers
                    .iter()
                    .filter(|n| !rules.contains_key(n))
                    .map(|n| *n)
                    .collect();
                for number_with_no_rule in numbers_with_no_rules {
                    new_numbers_order.push(number_with_no_rule);
                    numbers.swap_remove(
                        numbers
                            .iter()
                            .position(|n| *n == number_with_no_rule)
                            .unwrap(),
                    );
                }
                loop {
                    let mut numbers_to_push = Vec::new();
                    for number in numbers.iter() {
                        let rule = rules.get(number).unwrap();
                        if rule.iter().all(|prior| {
                            new_numbers_order.contains(prior) || !numbers.contains(prior)
                        }) {
                            numbers_to_push.push(*number);
                        }
                    }
                    for number in numbers_to_push {
                        new_numbers_order.push(number);
                        numbers.swap_remove(numbers.iter().position(|n| *n == number).unwrap());
                    }

                    if numbers.len() == 0 {
                        break;
                    }
                }

                sum += new_numbers_order[new_numbers_order.len() / 2];
            }
        }
    }

    println!("{}", sum);
}
