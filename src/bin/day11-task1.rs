fn main() {
    let input = include_str!("./input11.txt");

    let mut stones: Vec<i64> = input.split_whitespace().map(|n|n.parse().unwrap()).collect();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            let stone_as_string = stone.to_string();
            if stone == 0 {
                new_stones.push(1);
            }
            else if stone_as_string.len() % 2 == 0 {
                let left_part = &stone_as_string[..stone_as_string.len() / 2];
                let right_part = &stone_as_string[stone_as_string.len() / 2..];
                new_stones.push(left_part.parse().unwrap());
                new_stones.push(right_part.parse().unwrap());
            }
            else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{}", stones.len());
}
