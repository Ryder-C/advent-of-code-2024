use regex::Regex;

pub struct Prize {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    px: i64,
    py: i64,
}

#[aoc_generator(day13)]
pub fn generate_input(input: &str) -> Vec<Prize> {
    let mut prizes = Vec::new();

    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let mut lines = input.lines();
    while let Some(line_a) = lines.next() {
        let line_b = lines.next().unwrap();
        let line_p = lines.next().unwrap();

        let caps_a = re_button.captures(line_a).expect("Invalid A line format");
        let ax: usize = caps_a[1].parse().unwrap();
        let ay: usize = caps_a[2].parse().unwrap();

        let caps_b = re_button.captures(line_b).expect("Invalid B line format");
        let bx: usize = caps_b[1].parse().unwrap();
        let by: usize = caps_b[2].parse().unwrap();

        let caps_p = re_prize
            .captures(line_p)
            .expect("Invalid Prize line format");
        let px: i64 = caps_p[1].parse().unwrap();
        let py: i64 = caps_p[2].parse().unwrap();

        prizes.push(Prize {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        });
        lines.next();
    }
    prizes
}

#[aoc(day13, part1)]
pub fn part1(prizes: &[Prize]) -> usize {
    let mut cost = 0;
    for prize in prizes {
        let mut best_cost: Option<usize> = None;
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * prize.ax + b * prize.bx;
                let y = a * prize.ay + b * prize.by;

                if x as i64 == prize.px && y as i64 == prize.py {
                    let cost = 3 * a + b;
                    if let Some(bc) = best_cost {
                        best_cost = Some(bc.min(cost));
                    } else {
                        best_cost = Some(cost);
                    }
                }
            }
        }
        if let Some(bc) = best_cost {
            cost += bc;
        }
    }
    cost
}

#[aoc(day13, part2)]
pub fn part2(prizes: &[Prize]) -> i64 {
    const ADDITION: i64 = 10000000000000;
    let mut cost = 0;
    for prize in prizes {
        let det = (prize.ax * prize.by - prize.ay * prize.bx) as i64;
        if det != 0 {
            let px = prize.px + ADDITION;
            let py = prize.py + ADDITION;
            let num_a = px * prize.by as i64 - py * prize.bx as i64;
            let num_b = py * prize.ax as i64 - px * prize.ay as i64;

            if num_a % det != 0 || num_b % det != 0 {
                continue;
            }

            let a = num_a / det;
            let b = num_b / det;

            if a < 0 || b < 0 {
                continue;
            }

            cost += 3 * a + b;
        }
    }
    cost
}
