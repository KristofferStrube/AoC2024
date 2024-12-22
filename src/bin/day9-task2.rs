use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("./input9.txt");

    let map: Vec<u32> = input.chars().map(|n|n.to_digit(10).unwrap()).collect();

    let mut check_sum: i64 = 0;

    let mut expanded: Vec<i64> = Vec::new();
    let mut allocations: Vec<(u32, u32)> = Vec::new();

    let mut even = true;
    let mut id: i64 = 0;

    let mut i = 0;
    while i < map.len() {
        let mut n = map[i];
        if even {
            allocations.push((expanded.len().try_into().unwrap(), n));
            for _ in 0..n {
                expanded.push(id);
            }
            id += 1;
        } else {
            for _ in 0..n {
                expanded.push(-1);
            }
        }
        even = !even;
        i += 1;
    }

    allocations.reverse();
    for allocation in allocations {
        for from in 0..allocation.0 {
            let mut free_space = true;
            for i in 0..allocation.1 {
                if from + i >= expanded.len() as u32 || expanded[(from + i) as usize] != -1 {
                    free_space = false;
                    break;
                }
            }
            
            if free_space {
                for i in 0..allocation.1 {
                    expanded[(from + i) as usize] = expanded[(allocation.0 + i) as usize];
                    expanded[(allocation.0 + i) as usize] = -1;
                }
                break;
            }
        }
    }


    for i in 0..expanded.len() {
        if expanded[i] == -1 {
            continue;
        }
        check_sum += i as i64 * expanded[i];
    }

    println!("{}", check_sum);
}
