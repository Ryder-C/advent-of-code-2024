use std::collections::{HashMap, HashSet};

pub struct Map {
    antennas: Vec<(usize, usize, char)>,
    width: usize,
    height: usize,
}

#[aoc_generator(day8)]
pub fn generate_input(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (height, width) = (grid.len(), grid[0].len());

    let mut antennas = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.push((x, y, c));
            }
        }
    }

    Map {
        antennas,
        width,
        height,
    }
}

#[aoc(day8, part1)]
pub fn part1(map: &Map) -> usize {
    let mut freq_map = HashMap::new();
    for &(x, y, c) in &map.antennas {
        freq_map.entry(c).or_insert(vec![]).push((x, y));
    }

    let mut antinodes = HashSet::new();
    for (_freq, positions) in freq_map {
        let n = positions.len();
        if n < 2 {
            continue;
        }

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let c1x = 2 * x1 as isize - x2 as isize;
                let c1y = 2 * y1 as isize - y2 as isize;

                let c2x = 2 * x2 as isize - x1 as isize;
                let c2y = 2 * y2 as isize - y1 as isize;

                if c1x >= 0 && c1y >= 0 && (c1x as usize) < map.width && (c1y as usize) < map.height
                {
                    antinodes.insert((c1x as usize, c1y as usize));
                }

                if c2x >= 0 && c2y >= 0 && (c2x as usize) < map.width && (c2y as usize) < map.height
                {
                    antinodes.insert((c2x as usize, c2y as usize));
                }
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(map: &Map) -> usize {
    let mut freq_map = HashMap::new();
    for &(x, y, c) in &map.antennas {
        freq_map.entry(c).or_insert(vec![]).push((x, y));
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in freq_map {
        let n = positions.len();
        if n < 2 {
            continue;
        }

        for &(x, y) in &positions {
            antinodes.insert((x, y));
        }

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let g = gcd(dx, dy);
                let step_x = dx / g;
                let step_y = dy / g;

                let mut cx = x1 as isize;
                let mut cy = y1 as isize;
                loop {
                    cx += step_x;
                    cy += step_y;
                    if cx < 0 || cy < 0 || cx as usize >= map.width || cy as usize >= map.height {
                        break;
                    }
                    antinodes.insert((cx as usize, cy as usize));
                }

                let mut cx = x1 as isize;
                let mut cy = y1 as isize;
                loop {
                    cx -= step_x;
                    cy -= step_y;
                    if cx < 0 || cy < 0 || cx as usize >= map.width || cy as usize >= map.height {
                        break;
                    }
                    antinodes.insert((cx as usize, cy as usize));
                }
            }
        }
    }
    antinodes.len()
}

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
