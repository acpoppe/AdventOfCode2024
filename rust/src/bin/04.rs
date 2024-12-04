use self::Direction::*;
use std::slice::Iter;

use advent_of_code::aoc_helpers::read_chars;

advent_of_code::solution!(4);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] =
            [Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight];
        DIRECTIONS.iter()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let word_to_find = "XMAS";
    let matrix = read_chars(input);
    let mut found_count = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'X' {
                for dir in Direction::iterator() {
                    if check_for_word_in_direction(&matrix, x, y, word_to_find, dir) {
                        found_count += 1;
                    }
                }
            }
        }
    }
    Some(found_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn check_for_word_in_direction(
    matrix: &[Vec<char>],
    x: usize,
    y: usize,
    word: &str,
    direction: &Direction,
) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;
    let mut word_iter = word.chars();
    word_iter.next(); // Don't need to check for X again
    loop {
        match direction {
            Up => y -= 1,
            Down => y += 1,
            Left => x -= 1,
            Right => x += 1,
            UpLeft => {
                x -= 1;
                y -= 1;
            }
            UpRight => {
                x -= 1;
                y += 1;
            }
            DownLeft => {
                x += 1;
                y -= 1;
            }
            DownRight => {
                x += 1;
                y += 1;
            }
        }
        if x < 0 || y < 0 || y >= matrix.len() as i32 || x >= matrix[0].len() as i32 {
            return false;
        }
        if let Some(c) = word_iter.next() {
            if matrix[y as usize][x as usize] != c {
                return false;
            }
            if word_iter.as_str().is_empty() {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
