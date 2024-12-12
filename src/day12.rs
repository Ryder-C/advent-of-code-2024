use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(grid: &[Vec<char>]) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let dirs = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; n]; m];

    let mut total_cost = 0;
    for i in 0..m {
        for j in 0..n {
            if !visited[i][j] {
                visited[i][j] = true;
                let mut stack = vec![];
                stack.push((i, j));
                let mut region_cells = vec![];
                region_cells.push((i, j));

                while let Some((ci, cj)) = stack.pop() {
                    for &(di, dj) in &dirs {
                        let (ni, nj) = (ci as isize + di, cj as isize + dj);

                        if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                            let (ni, nj) = (ni as usize, nj as usize);
                            if !visited[ni][nj] && grid[ni][nj] == grid[i][j] {
                                visited[ni][nj] = true;
                                stack.push((ni, nj));
                                region_cells.push((ni, nj));
                            }
                        }
                    }
                }

                let area = region_cells.len();
                let perimeter = {
                    let mut perimter = 0;
                    for &(ci, cj) in &region_cells {
                        for &(di, dj) in &dirs {
                            let (ni, nj) = (ci as isize + di, cj as isize + dj);
                            if ni < 0 || ni >= m as isize || nj < 0 || nj >= n as isize {
                                perimter += 1;
                            } else {
                                let (ni, nj) = (ni as usize, nj as usize);
                                if grid[ni][nj] != grid[i][j] {
                                    perimter += 1;
                                }
                            }
                        }
                    }
                    perimter
                };
                total_cost += area * perimeter;
            }
        }
    }
    total_cost
}

// i apologize to anyone that is trying to read this
#[aoc(day12, part2)]
fn solve(grid: &[Vec<char>]) -> usize {
    // create a map of (i,j)->char
    let mut points = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            points.insert((i as i32, j as i32), c);
        }
    }

    // find connected regions of the same plant using BFS
    let mut visited = HashSet::new();
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut regions: Vec<HashSet<(i32, i32)>> = Vec::new();

    for (&p, &ch) in points.iter() {
        if !visited.contains(&p) {
            // BFS from p
            let mut q = VecDeque::new();
            q.push_back(p);
            visited.insert(p);
            let mut region = HashSet::new();
            region.insert(p);

            while let Some(cur) = q.pop_front() {
                for &d in &directions {
                    let np = (cur.0 + d.0, cur.1 + d.1);
                    if let Some(&c2) = points.get(&np) {
                        if c2 == ch && !visited.contains(&np) {
                            visited.insert(np);
                            q.push_back(np);
                            region.insert(np);
                        }
                    }
                }
            }

            regions.push(region);
        }
    }

    // dir * 1j (rotate left 90 degrees)
    fn mul_1j(dir: (i32, i32)) -> (i32, i32) {
        // (x,y)*(i) = (-y,x)
        let (x, y) = dir;
        (-y, x)
    }

    // dir * (1+1j) = (x - y, x + y)
    fn mul_1plus1j(dir: (i32, i32)) -> (i32, i32) {
        let (x, y) = dir;
        (x - y, x + y)
    }

    // circ function translation (i stole from the subreddit im sorry)
    // circ(ps) = sum over dirs and p in ps of the complex condition:
    // (dir+p not in ps)
    // and ((dir*1j+p not in ps) or ((dir*(1+1j)+p in ps) and (dir*(1+1j)+p in ps)))
    fn circ(ps: &HashSet<(i32, i32)>) -> usize {
        let mut count = 0;
        for &dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for &p in ps {
                let dp = (p.0 + dir.0, p.1 + dir.1);
                let d1j = {
                    // dir*1j
                    mul_1j(dir)
                };
                let d1j_p = (p.0 + d1j.0, p.1 + d1j.1);
                let d1plus1j = mul_1plus1j(dir);
                let d1plus1j_p = (p.0 + d1plus1j.0, p.1 + d1plus1j.1);

                let cond1 = !ps.contains(&dp);
                // ((dir*1j+p not in ps) or ((dir*(1+1j)+p in ps) and (dir*(1+1j)+p in ps)))
                let cond2 = (!ps.contains(&d1j_p))
                    || (ps.contains(&d1plus1j_p) && ps.contains(&d1plus1j_p));

                if cond1 && cond2 {
                    count += 1;
                }
            }
        }
        count
    }

    let mut total = 0;
    for s in regions {
        let c = circ(&s);
        total += s.len() * c;
    }

    total
}
