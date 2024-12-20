use std::collections::HashMap;

use advent_of_code::aoc_helpers::{read_lines, read_sections};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let input_sections = read_sections(input);
    let page_orders: HashMap<u64, Vec<u64>> = parse_page_orders(input_sections[0]);
    let updates = parse_updates(input_sections[1]);
    let valid_updates = updates
        .iter()
        .filter(|u| is_correctly_ordered(u, &page_orders))
        .collect::<Vec<&Vec<u64>>>();
    Some(valid_updates.iter().map(|u| u[u.len() / 2]).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_sections = read_sections(input);
    let page_orders: HashMap<u64, Vec<u64>> = parse_page_orders(input_sections[0]);
    let updates = parse_updates(input_sections[1]);
    let invalid_updates = updates
        .iter()
        .filter(|u| !is_correctly_ordered(u, &page_orders))
        .collect::<Vec<&Vec<u64>>>();
    let fixed_updates = invalid_updates
        .iter()
        .map(|u| fix_order(u, &page_orders))
        .collect::<Vec<Vec<u64>>>();
    Some(fixed_updates.iter().map(|u| u[u.len() / 2]).sum::<u64>())
}

fn fix_order(prepended_pages: &[u64], page_orders: &HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut fixed_order: Vec<u64> = vec![];
    (0..prepended_pages.len()).for_each(|i| {
        let mut inserted = false;
        for p in 0..fixed_order.len() {
            if page_orders
                .get(&prepended_pages[i])
                .unwrap_or(&vec![])
                .contains(&fixed_order[p])
            {
                fixed_order.insert(p, prepended_pages[i]);
                inserted = true;
                break;
            }
        }
        if !inserted {
            fixed_order.push(prepended_pages[i]);
        }
    });
    fixed_order
}

fn is_correctly_ordered(prepended_pages: &[u64], page_orders: &HashMap<u64, Vec<u64>>) -> bool {
    for i in 1..prepended_pages.len() {
        for p in 0..i {
            if page_orders
                .get(&prepended_pages[i])
                .unwrap_or(&vec![])
                .contains(&prepended_pages[p])
            {
                return false;
            }
        }
    }
    true
}

fn parse_updates(chunk: &str) -> Vec<Vec<u64>> {
    let lines = read_lines(chunk);
    lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn parse_page_orders(chunk: &str) -> HashMap<u64, Vec<u64>> {
    let lines = read_lines(chunk);
    let mut page_orders: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in lines {
        let (page, following) = parse_page_order(line);
        page_orders.entry(page).or_default().push(following);
    }
    page_orders
}

fn parse_page_order(line: &str) -> (u64, u64) {
    let parts = line.split("|").collect::<Vec<&str>>();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
