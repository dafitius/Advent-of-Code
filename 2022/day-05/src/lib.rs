use std::fs;
use regex::Regex;

pub fn parse_input(path: &str) -> Option<(Vec<Vec<char>>, Vec<(u8, usize, usize)>)> {
    let data = fs::read_to_string(path).ok()?;

    let parts: Vec<&str> = data.split("\r\n\r\n").collect();
    let mut stack_lines: Vec<&str> = parts[0].split("\n").collect();
    let storage_count = stack_lines.last().unwrap().chars().filter(|char| char.is_numeric()).collect::<Vec<char>>().len();
    stack_lines.pop();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; storage_count];

    for line in stack_lines.into_iter() {
        for (stack_num, crate_char) in line.chars().skip(1).step_by(4).enumerate() {
            if crate_char != ' ' {
                stacks[stack_num].push(crate_char);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut instructions: Vec<(u8, usize, usize)> = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for instruction_line in parts[1].split("\n") {
        let caps = re.captures(instruction_line).unwrap();

        let num_crates = caps.get(1).map_or(0, |n| (n.as_str().parse::<u8>().unwrap()));
        let from_stack = caps.get(2).map_or(0, |n| (n.as_str().parse::<usize>().unwrap())) - 1;
        let to_stack = caps.get(3).map_or(0, |n| (n.as_str().parse::<usize>().unwrap())) - 1;
        instructions.push((num_crates, from_stack, to_stack));
    }

    Some((stacks, instructions))
}

pub fn part1(parsed_input: &(Vec<Vec<char>>, Vec<(u8, usize, usize)>)) -> String {
    move_and_read_stacks(parsed_input, false)
}

pub fn part2(parsed_input: &(Vec<Vec<char>>, Vec<(u8, usize, usize)>)) -> String {
    move_and_read_stacks(parsed_input, true)
}

fn move_and_read_stacks((stacks, instructions): &(Vec<Vec<char>>, Vec<(u8, usize, usize)>), is_crane_9001: bool) -> String{
    let mut stacks = stacks.clone();

    for (num_crates, from_stack, to_stack) in instructions.iter() {
        let from_stack = &mut stacks[*from_stack];
        let mut picked_crates = from_stack.split_off(from_stack.len() - *num_crates as usize);
        if !is_crane_9001{
            picked_crates.reverse();
        }

        stacks[*to_stack].extend_from_slice(&picked_crates);
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<Vec<char>>().into_iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        if let Some(input) = parse_input("input/test_input.txt") {
            let result = part1(&input);
            assert_eq!(result, "CMZ");
        }
    }

    #[test]
    fn test_part2() {
        if let Some(input) = parse_input("input/test_input.txt") {
            let result = part2(&input);
            assert_eq!(result, "MCD");
        }
    }
}