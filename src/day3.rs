use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(data: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(data) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(data: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for mat in re.find_iter(data) {
        let s = mat.as_str();
        if s.starts_with("mul(") {
            if enabled {
                let cap = re.captures(s).unwrap();
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                sum += x * y;
            }
        } else if s == "do()" {
            enabled = true;
        } else if s == "don't()" {
            enabled = false;
        }
    }
    sum
}
