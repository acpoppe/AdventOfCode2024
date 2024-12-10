use std::collections::{HashMap, HashSet};

use advent_of_code::aoc_helpers::read_chars;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let map = read_chars(input);
    let antennas = parse_input(&map);
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for frequency in antennas.keys() {
        check_for_antinode(
            &mut antennas.get(frequency).unwrap().clone(),
            &mut antinodes,
            map[0].len() as i64,
            map.len() as i64,
        );
    }
    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = read_chars(input);
    let antennas = parse_input(&map);
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for frequency in antennas.keys() {
        check_for_antinode_pt2(
            &mut antennas.get(frequency).unwrap().clone(),
            &mut antinodes,
            map[0].len() as i64,
            map.len() as i64,
        );
    }
    Some(antinodes.len() as u64)
}

fn check_for_antinode_pt2(
    antenna_locations: &mut Vec<(i64, i64)>,
    antinodes: &mut HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) {
    while antenna_locations.len() > 1 {
        let (x1, y1) = antenna_locations.pop().unwrap();
        for (x2, y2) in antenna_locations.iter() {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let mut offset_x = 0;
            let mut offset_y = 0;
            while is_in_bounds((x1 + offset_x, y1 + offset_y), width, height) {
                antinodes.insert((x1 + offset_x, y1 + offset_y));
                offset_x += dx;
                offset_y += dy;
            }
            let mut offset_x = 0;
            let mut offset_y = 0;
            while is_in_bounds((x2 + offset_x, y2 + offset_y), width, height) {
                antinodes.insert((x2 + offset_x, y2 + offset_y));
                offset_x -= dx;
                offset_y -= dy;
            }
        }
    }
}

fn check_for_antinode(
    antenna_locations: &mut Vec<(i64, i64)>,
    antinodes: &mut HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) {
    while antenna_locations.len() > 1 {
        let (x1, y1) = antenna_locations.pop().unwrap();
        for (x2, y2) in antenna_locations.iter() {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let inv_dx = -dx;
            let inv_dy = -dy;
            if is_valid_location((x1 + dx, y1 + dy), (x1, y1), (*x2, *y2), width, height) {
                antinodes.insert((x1 + dx, y1 + dy));
            }
            if is_valid_location(
                (x2 + inv_dx, y2 + inv_dy),
                (x1, y1),
                (*x2, *y2),
                width,
                height,
            ) {
                antinodes.insert((x2 + inv_dx, y2 + inv_dy));
            }
            if is_valid_location((x1 - dx, y1 - dy), (x1, y1), (*x2, *y2), width, height) {
                antinodes.insert((x1 - dx, y1 - dy));
            }
            if is_valid_location(
                (x2 - inv_dx, y2 - inv_dy),
                (x1, y1),
                (*x2, *y2),
                width,
                height,
            ) {
                antinodes.insert((x2 - inv_dx, y2 - inv_dy));
            }
        }
    }
}

fn is_valid_location(
    check: (i64, i64),
    first: (i64, i64),
    second: (i64, i64),
    width: i64,
    height: i64,
) -> bool {
    is_in_bounds(check, width, height)
        && get_distance(first, check) != 0.0
        && get_distance(second, check) != 0.0
        && ((get_distance(first, check) / get_distance(second, check)) <= 0.5
            || (get_distance(first, check) / get_distance(second, check)) >= 2.0)
}

fn is_in_bounds(check: (i64, i64), width: i64, height: i64) -> bool {
    check.0 >= 0 && check.0 < width && check.1 >= 0 && check.1 < height
}

fn get_distance(first: (i64, i64), second: (i64, i64)) -> f64 {
    (((second.0 - first.0) as f64).powi(2) + ((second.1 - first.1) as f64).powi(2)).sqrt()
}

fn parse_input(map: &[Vec<char>]) -> HashMap<char, Vec<(i64, i64)>> {
    let mut result: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col.is_alphanumeric() {
                result.entry(*col).or_default().push((x as i64, y as i64));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
