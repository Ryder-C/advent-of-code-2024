use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    (left_list, right_list)
}

#[aoc(day1, part1)]
pub fn part1(list: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left_list = list.0.clone();
    let mut right_list = list.1.clone();

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(list: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut freqs = HashMap::new();
    for n in list.1.iter() {
        *freqs.entry(n).or_insert(0) += 1;
    }

    list.0
        .iter()
        .fold(0, |acc, n| acc + n * freqs.get(n).unwrap_or(&0))
}
