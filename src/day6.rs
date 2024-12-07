use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    start_direction: Direction,

    width: usize,
    height: usize,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Map {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let (height, width) = (grid.len(), grid[0].len());

    let mut start_x = 0;
    let mut start_y = 0;
    let mut start_direction = Direction::Up;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' && *c != '#' {
                start_x = j;
                start_y = i;
                start_direction = match *c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                };
                break;
            }
        }
    }

    Map {
        grid,
        start_x,
        start_y,
        start_direction,
        width,
        height,
    }
}

#[aoc(day6, part1)]
pub fn part1(map: &Map) -> usize {
    let mut x = map.start_x as i32;
    let mut y = map.start_y as i32;
    let mut curr_direction = map.start_direction;
    let mut visited = HashSet::new();

    loop {
        if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 {
            break;
        }

        visited.insert((x, y));

        let (fx, fy) = match curr_direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if fx < 0 || fx >= map.width as i32 || fy < 0 || fy >= map.height as i32 {
            break;
        }

        if map.grid[fy as usize][fx as usize] == '#' {
            curr_direction = match curr_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            x = fx;
            y = fy;
        }
    }
    visited.len()
}

// Scuffed solution but works somehow
#[aoc(day6, part1, scuffed)]
pub fn part1old(map: &Map) -> usize {
    let mut curr_direction = map.start_direction;
    let mut visited = HashSet::new();

    let mut x = map.start_x as i32;
    let mut y = map.start_y as i32;
    while x >= 0 && x < map.width as i32 && y >= 0 && y < map.height as i32 {
        let (i, j) = (y as usize, x as usize);
        if map.grid[i][j] == '#' {
            match curr_direction {
                Direction::Up => {
                    x += 1;
                    y += 1;
                    curr_direction = Direction::Right;
                }
                Direction::Right => {
                    x -= 1;
                    y += 1;
                    curr_direction = Direction::Down;
                }
                Direction::Down => {
                    x -= 1;
                    y -= 1;
                    curr_direction = Direction::Left;
                }
                Direction::Left => {
                    x += 1;
                    y -= 1;
                    curr_direction = Direction::Up;
                }
            }
        }
        visited.insert((x, y));
        match curr_direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }

    visited.len()
}

#[aoc(day6, part2)]
pub fn part2(map: &Map) -> usize {
    let mut valid_pos = 0;
    for oy in 0..map.height {
        for ox in 0..map.width {
            if map.grid[oy][ox] == '#' || (oy == map.start_y && ox == map.start_x) {
                continue;
            }

            let mut x = map.start_x as i32;
            let mut y = map.start_y as i32;
            let mut curr_direction = map.start_direction;
            let mut visited = HashSet::new();

            loop {
                if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 {
                    break;
                }

                if !visited.insert((x, y, curr_direction)) {
                    valid_pos += 1;
                    break;
                }

                let (fx, fy) = match curr_direction {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                };

                if fx < 0 || fx >= map.width as i32 || fy < 0 || fy >= map.height as i32 {
                    break;
                }

                if (map.grid[fy as usize][fx as usize] == '#')
                    || (fy as usize == oy && fx as usize == ox)
                {
                    curr_direction = match curr_direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                } else {
                    x = fx;
                    y = fy;
                }
            }
        }
    }
    valid_pos
}
