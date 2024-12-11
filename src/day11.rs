use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
}

pub fn transform_stone(stone: &str) -> Vec<String> {
    if stone == "0" {
        vec!["1".to_string()]
    } else if stone.len() % 2 == 0 {
        split_even_digits(stone)
    } else {
        let val = stone.parse::<u64>().unwrap();
        (val.checked_mul(2024)).map_or_else(
            || {
                let big_val = big_multiply_2024(stone);
                vec![big_val]
            },
            |res| vec![res.to_string()],
        )
    }
}

fn big_multiply_2024(num_str: &str) -> String {
    let val = num_str.parse::<u128>().unwrap();
    (val * 2024u128).to_string()
}

pub fn split_even_digits(num_str: &str) -> Vec<String> {
    let n = num_str.len();
    let left = &num_str[..n / 2];
    let right = &num_str[n / 2..];

    vec![
        left.parse::<u64>().unwrap().to_string(),
        right.parse::<u64>().unwrap().to_string(),
    ]
}

fn count_after_iterations(
    stone: &str,
    iterations: usize,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    let key = (stone.to_string(), iterations);
    if let Some(&cached_result) = memo.get(&key) {
        return cached_result;
    }

    let transformed = transform_stone(stone);
    let mut total_count = 0;
    for st in &transformed {
        total_count += count_after_iterations(st, iterations - 1, memo);
    }

    memo.insert(key, total_count);
    total_count
}

#[aoc(day11, part1)]
pub fn part1(stones: &[String]) -> usize {
    let mut current = stones.to_vec();
    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in &current {
            new_stones.extend(transform_stone(stone));
        }
        current = new_stones;
    }
    current.len()
}

#[aoc(day11, part2)]
pub fn part2(stones: &[String]) -> usize {
    let mut memo = HashMap::new();
    let iterations = 75;
    let mut total = 0;
    for stone in stones {
        total += count_after_iterations(stone, iterations, &mut memo);
    }
    total
}
