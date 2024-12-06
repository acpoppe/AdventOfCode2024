use std::collections::HashSet;

use advent_of_code::aoc_helpers::read_chars;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn next_step_offsets(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = read_chars(input);
    let obstacles = find_obstacles(&map);
    let mut current_guard_pos = find_starting_position(&map);
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    while current_guard_pos.0 >= 0 && current_guard_pos.0 < map[0].len() as isize && current_guard_pos.1 >= 0 && current_guard_pos.1 < map.len() as isize {
        turn_guard_if_needed(&mut current_guard_pos, &obstacles);
        move_guard(&mut current_guard_pos, &mut visited_positions);
    }
    Some(visited_positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn move_guard(guard_pos: &mut (isize, isize, Direction), visited_positions: &mut HashSet<(isize, isize)>) {
    visited_positions.insert((guard_pos.0, guard_pos.1));
    guard_pos.0 += guard_pos.2.next_step_offsets().0;
    guard_pos.1 += guard_pos.2.next_step_offsets().1;
}

fn turn_guard_if_needed(guard_position: &mut (isize, isize, Direction), obstacles: &HashSet<(isize, isize)>) {
    while obstacles.contains(&(guard_position.0 + guard_position.2.next_step_offsets().0, guard_position.1 + guard_position.2.next_step_offsets().1)) {
        guard_position.2 = guard_position.2.turn_right();
    }
}

fn find_obstacles(map: &[Vec<char>]) -> HashSet<(isize, isize)> {
    let mut obstacles = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.to_string() == "#" {
                obstacles.insert((x as isize, y as isize));
            }
        }
    }
    obstacles
}

fn find_starting_position(map: &[Vec<char>]) -> (isize, isize, Direction) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.to_string() == "^" {
                return (x as isize, y as isize, Direction::Up);
            } else if cell.to_string() == "v" {
                return (x as isize, y as isize, Direction::Down);
            } else if cell.to_string() == "<" {
                return (x as isize, y as isize, Direction::Left);
            } else if cell.to_string() == ">" {
                return (x as isize, y as isize, Direction::Right);
            }
        }
    }
    (0, 0, Direction::Up)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
