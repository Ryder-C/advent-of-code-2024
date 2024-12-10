#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[u8]) -> usize {
    let n = input.len();
    let mut i = 0;
    let mut file_id = 0;
    let mut blocks = vec![];

    while i < input.len() {
        let file_len = input[i] as usize;
        i += 1;

        if file_len > 0 {
            for _ in 0..file_len {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        }

        if i < n {
            let free_len = input[i] as usize;
            i += 1;

            for _ in 0..free_len {
                blocks.push(None);
            }
        }
    }

    while let Some(left_free) = blocks.iter().position(|b| b.is_none()) {
        let right_file = match blocks.iter().rposition(|b| b.is_some()) {
            Some(pos) => {
                if pos > left_free {
                    pos
                } else {
                    break;
                }
            }
            None => break,
        };

        blocks[left_free] = blocks[right_file].take();
    }

    let mut checksum = 0;
    for (i, b) in blocks.iter().enumerate() {
        if let Some(fi) = b {
            checksum += i * fi;
        }
    }
    checksum
}

#[aoc(day9, part2)]
pub fn part2(input: &[u8]) -> usize {
    let n = input.len();
    let mut i = 0;
    let mut file_id = 0;
    let mut blocks = vec![];

    while i < input.len() {
        let file_len = input[i] as usize;
        i += 1;

        if file_len > 0 {
            for _ in 0..file_len {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        }

        if i < n {
            let free_len = input[i] as usize;
            i += 1;

            for _ in 0..free_len {
                blocks.push(None);
            }
        }
    }

    let total_files = file_id;
    let mut positions = file_positions(&blocks);
    for id in (0..total_files).rev() {
        let (start, end) = positions[id];
        let file_len = end - start + 1;

        if let Some((dst_start, dst_end)) = find_free(&blocks, file_len, start) {
            let mut file_blocks = Vec::with_capacity(file_len);
            for block in blocks.iter_mut().take(end + 1).skip(start) {
                file_blocks.push(block.take());
            }

            blocks[dst_start..(dst_end + 1)]
                .copy_from_slice(&file_blocks[..((dst_end - dst_start) + 1)]);

            positions[id] = (dst_start, dst_end);
        }
    }
    let mut checksum = 0;
    for (i, b) in blocks.iter().enumerate() {
        if let Some(fi) = b {
            checksum += i * fi;
        }
    }
    checksum
}

fn file_positions(blocks: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    let max_id = blocks.iter().filter_map(|b| *b).max().unwrap_or(0);
    for id in 0..=max_id {
        if let Some(start) = blocks.iter().position(|&b| b == Some(id)) {
            let mut end = start;
            while end + 1 < blocks.len() && blocks[end + 1] == Some(id) {
                end += 1;
            }
            positions.push((start, end));
        } else {
            positions.push((0, 0));
        }
    }
    positions
}

fn find_free(
    blocks: &[Option<usize>],
    file_len: usize,
    file_start: usize,
) -> Option<(usize, usize)> {
    let mut seg_start = None;
    for (i, block) in blocks.iter().enumerate().take(file_start) {
        if block.is_none() {
            if seg_start.is_none() {
                seg_start = Some(i);
            }
        } else if let Some(s) = seg_start {
            let len = i - s;
            if len >= file_len {
                return Some((s, s + file_len - 1));
            }
            seg_start = None;
        }
    }

    if let Some(s) = seg_start {
        let len = file_start - s;
        if len >= file_len {
            return Some((s, s + file_len - 1));
        }
    }
    None
}
