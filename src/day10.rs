use std::collections::{HashSet, VecDeque};

#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(grid: &[Vec<u8>]) -> usize {
    let m = grid.len();
    let n = grid[0].len();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut trailheads = vec![];
    for (i, row) in grid.iter().enumerate().take(m) {
        for (j, height) in row.iter().enumerate().take(n) {
            if *height == 0 {
                trailheads.push((i, j));
            }
        }
    }

    let mut total_score = 0;
    for &(si, sj) in &trailheads {
        let mut visited = vec![vec![false; n]; m];
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((si, sj));
        visited[si][sj] = true;

        while let Some((i, j)) = queue.pop_front() {
            if grid[i][j] == 9 {
                reachable.insert((i, j));
            } else {
                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if !visited[ni][nj] && grid[ni][nj] == grid[i][j] + 1 {
                        visited[ni][nj] = true;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
        total_score += reachable.len();
    }
    total_score
}

// I basically just copied the code from part1 lol
#[aoc(day10, part2)]
pub fn part2(grid: &[Vec<u8>]) -> usize {
    let m = grid.len();
    let n = grid[0].len();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut trailheads = vec![];
    for (i, row) in grid.iter().enumerate().take(m) {
        for (j, height) in row.iter().enumerate().take(n) {
            if *height == 0 {
                trailheads.push((i, j));
            }
        }
    }

    let mut total_rating = 0;
    for &(si, sj) in &trailheads {
        let mut rating = 0;
        let mut queue = VecDeque::new();
        queue.push_back((si, sj));

        while let Some((i, j)) = queue.pop_front() {
            if grid[i][j] == 9 {
                rating += 1;
            } else {
                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if grid[ni][nj] == grid[i][j] + 1 {
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
        total_rating += rating;
    }
    total_rating
}
