fn main() {
    let input = include_str!("./input9.txt");

    let map: Vec<u32> = input.chars().map(|n|n.to_digit(10).unwrap()).collect();

    let mut check_sum: i64 = 0;

    let mut expanded: Vec<i64> = Vec::new();

    let mut even = true;
    let mut id: i64 = 0;

    for n in map {
        if even {
            for _ in 0..n {
                expanded.push(id);
            }
        } else {
            for _ in 0..n {
                expanded.push(-1);
            }
        }
        even = !even;
        if even {
            id += 1;
        }
    }

    let mut i = 0;
    let mut j = expanded.len() - 1;

    loop {
        while expanded[i] != -1 {
            i += 1;
        }
        while expanded[j] == -1 {
            j -= 1;
        }
        if i > j {
            break;
        }
        expanded[i] = expanded[j];
        expanded[j] = -1;
    }

    for i in 0..expanded.len() {
        if expanded[i] == -1 {
            break;
        }
        check_sum += i as i64 * expanded[i];
    }

    println!("{}", check_sum);
}
