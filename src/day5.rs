use std::collections::{HashMap, VecDeque};

pub struct InputData {
    pub rules: Vec<(i32, i32)>,
    pub updates: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputData {
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let rules = rules_section
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        }).collect::<Vec<(i32, i32)>>();

    let updates = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

    InputData { rules, updates }
}


#[aoc(day5, part1)]
pub fn part1(input: &InputData) -> i32 {
    let mut total = 0;

    for update in &input.updates {
        let mut page_index = HashMap::new();
        for (idx, &page) in update.iter().enumerate() {
            page_index.insert(page, idx);
        }

        let mut valid = true;
        for &(x, y) in &input.rules {
            if let (Some(&idx_x), Some(&idx_y)) = (page_index.get(&x), page_index.get(&y)) {
                if idx_x >= idx_y {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            total += update[update.len() / 2];
        }
    }
    total
}

#[aoc(day5, part2)]
pub fn part2(input: &InputData) -> i32 {
    let mut total = 0;
    
    for update in &input.updates {
        let mut page_index = HashMap::new();
        for (idx, &page) in update.iter().enumerate() {
            page_index.insert(page, idx);
        }

        let mut valid = true;
        for &(x, y) in &input.rules {
            if let (Some(&idx_x), Some(&idx_y)) = (page_index.get(&x), page_index.get(&y)) {
                if idx_x >= idx_y {
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            let pages = update.clone();
            let mut graph = HashMap::new();
            let mut in_degree = HashMap::new();

            for &page in &pages {
                graph.entry(page).or_insert(vec![]);
                in_degree.entry(page).or_insert(0);
            }

            for &(x, y) in &input.rules {
                if pages.contains(&x) && pages.contains(&y) {
                    graph.get_mut(&x).unwrap().push(y);
                    *in_degree.entry(y).or_insert(0) += 1;
                }
            }

            // Kahn's algorithm
            let mut queue = VecDeque::new();
            for &page in &pages {
                if *in_degree.get(&page).unwrap() == 0 {
                    queue.push_back(page);
                }
            }

            let mut sorted_pages = vec![];
            while let Some(page) = queue.pop_front() {
                sorted_pages.push(page);

                for &neighbor in &graph[&page] {
                    *in_degree.get_mut(&neighbor).unwrap() -= 1;
                    if in_degree[&neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }

            if sorted_pages.len() != pages.len() {
                unreachable!()
            }

            total += sorted_pages[sorted_pages.len() / 2];
        }
    }
    total
}
