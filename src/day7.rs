#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let target = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let nums = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            (target, nums)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(i64, Vec<i64>)]) -> i64 {
    let mut total = 0;
    for (target, nums) in input {
        if nums.len() == 1 {
            if nums[0] == *target {
                total += target;
            }
            continue;
        }

        let n_ops = nums.len() - 1;
        for mask in 0..(1 << n_ops) {
            let mut ops = vec![];
            for i in 0..n_ops {
                ops.push(((mask >> i) & 1) == 1);
            }
            let val = evaluate_expression(nums, &ops);
            if val == *target {
                total += target;
                break;
            }
        }
    }
    total
}

fn evaluate_expression(nums: &[i64], ops: &[bool]) -> i64 {
    let mut result = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        if op {
            result *= nums[i + 1];
        } else {
            result += nums[i + 1];
        }
    }
    result
}

#[aoc(day7, part2)]
pub fn part2(input: &[(i64, Vec<i64>)]) -> i64 {
    let mut total = 0;
    for (target, nums) in input {
        if nums.len() == 1 {
            if nums[0] == *target {
                total += target;
            }
            continue;
        }

        let n_ops = nums.len() - 1;
        for mask in 0..3_u32.pow(n_ops as u32) {
            let mut ops = vec![];
            let mut temp_mask = mask;
            for _ in 0..n_ops {
                ops.push((temp_mask % 3) as u8);
                temp_mask /= 3;
            }
            let val = evaluate_expression_concat(nums, &ops);
            if val == *target {
                total += target;
                break;
            }
        }
    }
    total
}

fn evaluate_expression_concat(nums: &[i64], ops: &[u8]) -> i64 {
    let mut result = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        let next_val = nums[i + 1];
        match op {
            0 => {
                result += next_val;
            }
            1 => {
                result *= next_val;
            }
            2 => {
                let concat_str = format!("{}{}", result, next_val);
                result = concat_str.parse::<i64>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    result
}
