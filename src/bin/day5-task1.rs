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
            let parts: Vec<i32> = line.split('|').map(|p|p.parse().unwrap()).collect();
            let rule = rules.entry(parts[1]).or_insert(Vec::new());
            rule.push(parts[0]);
        }
        else {
            let numbers: Vec<i32> = line.split(',').map(|p|p.parse().unwrap()).collect();
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
            if valid {
                sum += numbers[numbers.len()/2];
            }
        }
    }

    println!("{}", sum);
}