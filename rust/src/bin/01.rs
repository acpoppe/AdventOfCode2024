use std::collections::HashMap;

use advent_of_code::aoc_helpers::read_lines;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    let mut diffs: Vec<i64> = vec![];
    read_lines(input).iter().for_each(|line| {
        let pair = line.split_whitespace().collect::<Vec<&str>>();
        left.push(pair[0].parse::<i64>().unwrap());
        right.push(pair[1].parse::<i64>().unwrap());
    });
    left.sort();
    right.sort();
    for i in 0..left.len() {
        diffs.push((left[i] - right[i]).abs());
    }
    Some(diffs.iter().sum::<i64>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left: Vec<u64> = vec![];
    let mut right: HashMap<&str, u64> = HashMap::new();
    read_lines(input).iter().for_each(|line| {
        let pair = line.split_whitespace().collect::<Vec<&str>>();
        left.push(pair[0].parse::<u64>().unwrap());
        if right.contains_key(&pair[1]) {
            right.insert(pair[1], right.get(&pair[1]).unwrap() + 1);
        } else {
            right.insert(pair[1], 1);
        }
    });
    Some(
        left.iter()
            .map(|l| {
                if right.contains_key(l.to_string().as_str()) {
                    right.get(l.to_string().as_str()).unwrap() * l
                } else {
                    0
                }
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
