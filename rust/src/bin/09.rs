advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone)]
enum Block {
    Filled(u64, u64),
    Empty(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    compact(&mut map);
    combine_at_end(&mut map);
    Some(calculate_return(&map))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn combine_at_end(map: &mut Vec<Block>) {
    let first = map[map.len() - 2];
    let second = map[map.len() - 1];
    if let (Block::Filled(x, y), Block::Filled(a, b)) = (first, second) {
        if x == a {
            map.pop();
            map.pop();
            map.push(Block::Filled(x, y + b));
        }
    }
}

fn calculate_return(map: &[Block]) -> u64 {
    let mut i = 0;
    let mut total = 0;
    for block in map {
        match block {
            Block::Filled(n, j) => {
                for _ in 0..*j {
                    total += i * n;
                    i += 1;
                }
            },
            _ => continue,
        }
    }
    total
}

fn compact(map: &mut Vec<Block>) {
    let mut i = 0;
    while contains_empty(map) {
        clear_empty_from_end(map);
        match map[i] {
            Block::Filled(_, _) => i += 1,
            _ => compact_block(map, i),
        }
    }
}

fn clear_empty_from_end(map: &mut Vec<Block>) {
    for i in (0..map.len()).rev() {
        match map[i] {
            Block::Empty(_) => map.remove(i),
            _ => break,
        };
    }
}

fn compact_block(map: &mut Vec<Block>, i: usize) {
    let (x, y) = match find_last_filled(map) {
        Some(Block::Filled(x, y)) => (x, y),
        _ => return,
    };

    let empty_blocks = match map[i] {
        Block::Empty(n) => n,
        _ => return,
    };

    match empty_blocks.cmp(&y) {
        std::cmp::Ordering::Less => {
            map[i] = Block::Filled(x, empty_blocks);
            map.push(Block::Filled(x, y - empty_blocks));
        },
        std::cmp::Ordering::Equal => {
            map[i] = Block::Filled(x, y);
        },
        std::cmp::Ordering::Greater => {
            map[i] = Block::Empty(empty_blocks - y);
            map.insert(i, Block::Filled(x, y));
        },
    }
}

fn contains_empty(map: &Vec<Block>) -> bool {
    for block in map {
        match block {
            Block::Empty(_) => return true,
            _ => continue,
        }
    }
    false
}

fn find_last_filled(map: &mut Vec<Block>) -> Option<Block> {
    for i in (0..map.len()).rev() {
        match map[i] {
            Block::Filled(_, _) => return Some(map.remove(i)),
            _ => {
                map.remove(i);
                continue;
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut map: Vec<Vec<Block>> = input.lines().map(|l| {
        l.chars().enumerate().map(|(i, c)| {
            if i % 2 == 0 {
                return Block::Filled((i / 2) as u64, c.to_digit(10).unwrap() as u64);
            }
            Block::Empty(c.to_digit(10).unwrap() as u64)
        }).collect::<Vec<Block>>()
    }).collect();

    map.remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}