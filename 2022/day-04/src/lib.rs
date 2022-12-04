use std::fs;

pub fn parse_input(path: &str) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
    let data = fs::read_to_string(path).ok()?;

    //ho ho horror
    let lines: Vec<(Vec<u8>, Vec<u8>)> = data.lines().map(|i| {
        let task_lists: Vec<Vec<u8>> = i.split(',').map(|i| {
            let bounds: Vec<u8> = i.trim_end().split("-").map(|bound| {
                bound.trim_end().parse::<u8>().unwrap()
            }).collect();
            (bounds[0]..=bounds[1]).collect::<Vec<u8>>()
        }).collect();
        (task_lists[0].to_vec(), task_lists[1].to_vec())
    }).collect();

    Some(lines)
}

pub fn part1(elf_pairs: &Vec<(Vec<u8>, Vec<u8>)>) -> u64 {
    let mut subset_counter: u64 = 0;
    for elf_pair in elf_pairs {
        if elf_pair.0.iter().all(|item| elf_pair.1.contains(item)) {
            subset_counter += 1;
            continue;
        }
        subset_counter += elf_pair.1.iter().all(|item| elf_pair.0.contains(item)) as u64;
    }
    subset_counter
}

pub fn part2(elf_pairs: &Vec<(Vec<u8>, Vec<u8>)>) -> u64 {
    let mut overlap_counter: u64 = 0;
    for elf_pair in elf_pairs {
        if elf_pair.0.iter().any(|item| elf_pair.1.contains(item)) {
            overlap_counter += 1;
        }
    }
    overlap_counter
}