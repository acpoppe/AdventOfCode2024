use self::Direction::*;
use std::slice::Iter;

use advent_of_code::aoc_helpers::read_chars;

advent_of_code::solution!(4);

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn diagonal_iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [UpLeft, UpRight, DownLeft, DownRight];
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
    let word_to_find = "MAS";
    let matrix = read_chars(input);
    let mut found_count = 0;
    let mut ignore_coords: Vec<(usize, usize, Direction)> = vec![];
    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'M' {
                for dir in Direction::diagonal_iterator() {
                    if check_for_word_in_direction_ignoring_coords(
                        &matrix,
                        x,
                        y,
                        word_to_find,
                        dir,
                        &ignore_coords,
                    ) {
                        if matches!(dir, UpLeft)
                            && (check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x - 2,
                                y,
                                word_to_find,
                                &UpRight,
                                &ignore_coords,
                            ) || check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x,
                                y - 2,
                                word_to_find,
                                &DownLeft,
                                &ignore_coords,
                            ))
                        {
                            found_count += 1;
                            add_ignored_coords(
                                x,
                                y,
                                x - 2,
                                y - 2,
                                [UpLeft, DownRight, DownLeft, UpRight],
                                &mut ignore_coords,
                            );
                        } else if matches!(dir, DownRight)
                            && (check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x + 2,
                                y,
                                word_to_find,
                                &DownLeft,
                                &ignore_coords,
                            ) || check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x,
                                y + 2,
                                word_to_find,
                                &UpRight,
                                &ignore_coords,
                            ))
                        {
                            found_count += 1;
                            add_ignored_coords(
                                x,
                                y,
                                x + 2,
                                y + 2,
                                [DownRight, UpLeft, UpRight, DownLeft],
                                &mut ignore_coords,
                            );
                        } else if matches!(dir, UpRight)
                            && (check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x + 2,
                                y,
                                word_to_find,
                                &UpLeft,
                                &ignore_coords,
                            ) || check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x,
                                y - 2,
                                word_to_find,
                                &DownRight,
                                &ignore_coords,
                            ))
                        {
                            found_count += 1;
                            add_ignored_coords(
                                x,
                                y,
                                x + 2,
                                y - 2,
                                [UpRight, DownLeft, DownRight, UpLeft],
                                &mut ignore_coords,
                            );
                        } else if matches!(dir, DownLeft)
                            && (check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x - 2,
                                y,
                                word_to_find,
                                &DownRight,
                                &ignore_coords,
                            ) || check_for_word_in_direction_ignoring_coords(
                                &matrix,
                                x,
                                y + 2,
                                word_to_find,
                                &UpLeft,
                                &ignore_coords,
                            ))
                        {
                            found_count += 1;
                            add_ignored_coords(
                                x,
                                y,
                                x - 2,
                                y + 2,
                                [DownLeft, UpRight, UpLeft, DownRight],
                                &mut ignore_coords,
                            );
                        }
                    }
                }
            }
        }
    }
    Some(found_count)
}

fn add_ignored_coords(
    x: usize,
    y: usize,
    opposite_x: usize,
    opposite_y: usize,
    ignore_dirs: [Direction; 4],
    ignored_coords: &mut Vec<(usize, usize, Direction)>,
) {
    ignored_coords.push((x, y, ignore_dirs[0]));
    ignored_coords.push((opposite_x, opposite_y, ignore_dirs[1]));
    ignored_coords.push((x, opposite_y, ignore_dirs[2]));
    ignored_coords.push((opposite_x, y, ignore_dirs[3]));
}

fn check_for_word_in_direction_ignoring_coords(
    matrix: &[Vec<char>],
    x: usize,
    y: usize,
    word: &str,
    direction: &Direction,
    ignore_coords: &[(usize, usize, Direction)],
) -> bool {
    if ignore_coords.contains(&(x, y, *direction)) {
        return false;
    }
    check_for_word_in_direction(matrix, x, y, word, direction)
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
    if let Some(c) = word_iter.next() {
        if matrix[y as usize][x as usize] != c {
            return false;
        }
    }
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
                x += 1;
                y -= 1;
            }
            DownLeft => {
                x -= 1;
                y += 1;
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
        assert_eq!(result, Some(9));
    }
}
