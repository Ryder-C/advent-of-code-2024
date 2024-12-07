#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(grid: &[Vec<char>]) -> i32 {
    let directions = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    let word = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            for &(dx, dy) in &directions {
                let mut k = 0;
                let mut x = i;
                let mut y = j;
                while k < word.len() {
                    if x < 0 || x >= n || y < 0 || y >= m || grid[x as usize][y as usize] != word[k]
                    {
                        break;
                    }
                    x += dx;
                    y += dy;
                    k += 1;
                }
                if k == word.len() {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(grid: &[Vec<char>]) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;

    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            let nw_se = [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
            let nw_se_matches = (nw_se[0] == 'M' && nw_se[1] == 'A' && nw_se[2] == 'S')
                || (nw_se[0] == 'S' && nw_se[1] == 'A' && nw_se[2] == 'M');

            let ne_sw = [grid[i - 1][j + 1], grid[i][j], grid[i + 1][j - 1]];
            let ne_sw_matches = (ne_sw[0] == 'M' && ne_sw[1] == 'A' && ne_sw[2] == 'S')
                || (ne_sw[0] == 'S' && ne_sw[1] == 'A' && ne_sw[2] == 'M');

            if nw_se_matches && ne_sw_matches {
                count += 1;
            }
        }
    }
    count
}
