use advent_of_code::aoc_helpers::read_lines;

advent_of_code::solution!(2);

#[derive(Clone)]
enum LineChange {
    Increasing,
    Decreasing,
}

enum Safety {
    Safe,
    Unsafe,
}

fn check_for_safe_line(line: &Vec<i64>, can_remove_value: bool) -> Safety {
    let mut i = 0;
    let mut j = 1;
    let mut line_change: Option<LineChange> = None;
    loop {
        let difference = line[i] - line[j];
        if difference.abs() > 3 || difference.abs() < 1 {
            if !can_remove_value {
                break Safety::Unsafe;
            }
            let mut k = 0;
            break loop {
                let mut without_k = line.clone();
                without_k.remove(k);
                if matches!(check_for_safe_line(&without_k, false), Safety::Safe) {
                    break Safety::Safe;
                }
                if k == line.len() - 1 {
                    break Safety::Unsafe;
                }
                k += 1;
            };
        }
        if difference > 0 && line_change.is_none() {
            line_change = Some(LineChange::Decreasing);
        } else if difference < 0 && line_change.is_none() {
            line_change = Some(LineChange::Increasing);
        } else {
            if difference > 0 && matches!(line_change.clone().unwrap(), LineChange::Increasing) {
                if !can_remove_value {
                    break Safety::Unsafe;
                }
                let mut k = 0;
                break loop {
                    let mut without_k = line.clone();
                    without_k.remove(k);
                    if matches!(check_for_safe_line(&without_k, false), Safety::Safe) {
                        break Safety::Safe;
                    }
                    if k == line.len() - 1 {
                        break Safety::Unsafe;
                    }
                    k += 1;
                };
            } else if difference < 0
                && matches!(line_change.clone().unwrap(), LineChange::Decreasing)
            {
                if !can_remove_value {
                    break Safety::Unsafe;
                }
                let mut k = 0;
                break loop {
                    let mut without_k = line.clone();
                    without_k.remove(k);
                    if matches!(check_for_safe_line(&without_k, false), Safety::Safe) {
                        break Safety::Safe;
                    }
                    if k == line.len() - 1 {
                        break Safety::Unsafe;
                    }
                    k += 1;
                };
            }
        }
        if j == line.len() - 1 {
            break Safety::Safe;
        }
        i += 1;
        j += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = read_lines(input)
        .into_iter()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    Some(
        lines
            .iter()
            .map(|line| check_for_safe_line(line, false))
            .filter(|x| matches!(x, Safety::Safe))
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = read_lines(input)
        .into_iter()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    Some(
        lines
            .iter()
            .map(|line| check_for_safe_line(line, true))
            .filter(|x| matches!(x, Safety::Safe))
            .count()
            .try_into()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
