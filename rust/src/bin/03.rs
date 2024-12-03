use regex::Regex;

advent_of_code::solution!(3);

fn execute_program(input: &str, reg: Regex) -> u64 {
    let cmds = reg.find_iter(input)
        .map(|cmd| cmd.as_str())
        .collect::<Vec<&str>>();
    let mut executable_cmds: Vec<&str> = vec![];

    let mut stop_execution = false;

    for i in 0..cmds.len() {
        if cmds[i] == "do()" {
            stop_execution = false;
        } else if cmds[i] == "don't()" {
            stop_execution = true;
        } else if !stop_execution {
            executable_cmds.push(cmds[i]);
        }
    }
        
    executable_cmds.into_iter().map(|cmd| {
        let nums: Vec<u64> = cmd.split("(").collect::<Vec<&str>>()[1]
            .split(")")
            .collect::<Vec<&str>>()[0]
            .split(",")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        nums[0] * nums[1]
    })
    .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let reg: Regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    Some(execute_program(input, reg))
}

pub fn part_two(input: &str) -> Option<u64> {
    let reg: Regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    Some(execute_program(input, reg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
