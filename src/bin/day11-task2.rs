use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input11.txt");

    let stones: Vec<i64> = input.split_whitespace().map(|n|n.parse().unwrap()).collect();

    let mut cache: HashMap<(i64, i32), i64> = HashMap::new();

    let mut numbers_to_cache: Vec<(i64, i32)> = Vec::new();

    for stone in stones.iter() {
        for i in 1..76 {
            get_length(&mut cache, &mut numbers_to_cache, *stone, i);
            while numbers_to_cache.len() > 0 {
                let number = numbers_to_cache.pop().unwrap();
                get_length(&mut cache, &mut numbers_to_cache, number.0, number.1);
            }
        }
    }

    let mut length = 0;
    for stone in stones {
        length += get_length(&mut cache, &mut numbers_to_cache, stone, 75);
    }

    println!("{}", length);
}

fn get_length(cache: &mut HashMap<(i64, i32), i64>, numbers_to_cache: &mut Vec<(i64, i32)>, stone: i64, iterations: i32) -> i64 {
    let mut stack = VecDeque::new();
    stack.push_back((stone, iterations));
    let mut length = 0;
    while stack.back().is_some() {
        let stone = stack.pop_back().unwrap();
        let cache_value = cache.get(&stone);
        if cache_value.is_some() {
            length += cache_value.unwrap();
        }
        else if stone.1 == 0 {
            length += 1;
            cache.insert(stone, 1);
        }
        else {
            numbers_to_cache.push(stone);
            let stone_as_string = stone.0.to_string();
            if stone.0 == 0 {
                stack.push_back((1, stone.1 - 1));
            }
            else if stone_as_string.len() % 2 == 0 {
                let left_part = &stone_as_string[..stone_as_string.len() / 2];
                let right_part = &stone_as_string[stone_as_string.len() / 2..];
                stack.push_back((left_part.parse().unwrap(), stone.1 - 1));
                stack.push_back((right_part.parse().unwrap(), stone.1 - 1));
            }
            else {
                stack.push_back((stone.0 * 2024, stone.1 - 1));
            }
        }
    }
    cache.insert((stone, iterations), length);
    return length;
}