#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for l in input.lines() {
        let first = l
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        ans.push(first);
    }
    ans
}

#[aoc(day2, part1)]
fn part_1(data: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for line in data {
        let increasing = line.iter().enumerate().skip(1).all(|(i, x)| {
            let diff = (*x - line[i - 1]).abs();
            *x > line[i - 1] && (1..=3).contains(&diff)
        });
        let decreasing = line.iter().enumerate().skip(1).all(|(i, x)| {
            let diff = (*x - line[i - 1]).abs();
            *x < line[i - 1] && (1..=3).contains(&diff)
        });
        if increasing || decreasing {
            ans += 1;
        }
    }
    ans
}

// Probably a much faster way to do this
#[aoc(day2, part2)]
fn part_2(data: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for line in data {
        let is_safe = |line: &[i32]| -> bool {
            let increasing = line.iter().enumerate().skip(1).all(|(i, &x)| {
                let diff = (x - line[i - 1]).abs();
                x > line[i - 1] && (1..=3).contains(&diff)
            });
            let decreasing = line.iter().enumerate().skip(1).all(|(i, &x)| {
                let diff = (x - line[i - 1]).abs();
                x < line[i - 1] && (1..=3).contains(&diff)
            });
            increasing || decreasing
        };

        if is_safe(line) {
            ans += 1;
            continue;
        }

        let mut found_safe = false;
        for i in 0..line.len() {
            let mut modified_line = line.clone();
            modified_line.remove(i);
            if is_safe(&modified_line) {
                found_safe = true;
                break;
            }
        }

        if found_safe {
            ans += 1;
        }
    }

    ans
}
