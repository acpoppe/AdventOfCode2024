use std::vec::IntoIter;

use advent_of_code::aoc_helpers::read_lines;

advent_of_code::solution!(7);

enum Operations {
    Add,
    Multiply,
    Concatenate,
}

impl Operations {
    fn use_operation(&self, a: u64, b: u64) -> u64 {
        match self {
            Operations::Add => a + b,
            Operations::Multiply => a * b,
            Operations::Concatenate => {
                let mut c = b;
                let mut shift = 1;
                while c > 0 {
                    shift *= 10;
                    c /= 10;
                }
                a * shift + b
            }
        }
    }

    fn iter() -> IntoIter<Operations> {
        vec![Operations::Add, Operations::Multiply].into_iter()
    }

    fn iter_with_concat() -> IntoIter<Operations> {
        vec![
            Operations::Add,
            Operations::Multiply,
            Operations::Concatenate,
        ]
        .into_iter()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_lines(input);
    let mut total = 0;
    for (target, operands) in lines {
        if check_line(target, &operands, None, false) {
            total += target;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_lines(input);
    let mut total = 0;
    for (target, operands) in lines {
        if check_line(target, &operands, None, true) {
            total += target;
        }
    }
    Some(total)
}

fn parse_lines(input: &str) -> Vec<(u64, Vec<u64>)> {
    read_lines(input)
        .into_iter()
        .map(|x| {
            let p = x.split(":").collect::<Vec<&str>>();
            let o = p[1]
                .split_whitespace()
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (p[0].parse::<u64>().unwrap(), o)
        })
        .collect::<Vec<(u64, Vec<u64>)>>()
}

fn check_line(target: u64, operands: &[u64], total: Option<u64>, with_concat: bool) -> bool {
    if operands.is_empty() {
        if let Some(t) = total {
            return target == t;
        }
        return false;
    }

    if total > Some(target) {
        return false;
    }

    let iter = if with_concat {
        Operations::iter_with_concat()
    } else {
        Operations::iter()
    };

    for operation in iter {
        let mut new_total: u64;
        let next_index: usize;
        if total.is_none() {
            new_total = operands[0];
            new_total = operation.use_operation(new_total, operands[1]);
            next_index = 2;
        } else {
            new_total = total.unwrap();
            new_total = operation.use_operation(new_total, operands[0]);
            next_index = 1;
        }
        if check_line(
            target,
            &operands[next_index..],
            Some(new_total),
            with_concat,
        ) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
