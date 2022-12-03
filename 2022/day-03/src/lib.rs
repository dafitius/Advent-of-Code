use std::fs;

pub fn parse_input(path: &str) -> Option<Vec<Vec<char>>> {
    let data = fs::read_to_string(path).ok()?;

    let lines: Vec<Vec<char>> = data.split("\n").map(|i| {
        i.trim().chars().collect::<Vec<char>>()
    }).collect();

    Some(lines)
}

pub fn part1(rucksacks: &Vec<Vec<char>>) -> u64 {
    let mut dup_count: u64 = 0;

    for rucksack in rucksacks.iter() {
        let comp_1: &[char] = &rucksack[0..rucksack.len() / 2];
        let comp_2: &[char] = &rucksack[rucksack.len() / 2..];

        for item in comp_2 {
            if comp_1.contains(&item) {
                if item.is_uppercase() { dup_count += *item as u64 - 38 } else { dup_count += *item as u64 - 96 }
                break;
            }
        }
    }
    dup_count
}

pub fn part2(rucksacks: &Vec<Vec<char>>) -> u64 {
    let mut badge_count: u64 = 0;

    for rucksack_group in rucksacks.chunks(3) {
        for item in rucksack_group[2].iter() {
            if rucksack_group[0].contains(&item) && rucksack_group[1].contains(&item) {
                if item.is_uppercase() { badge_count += *item as u64 - 38 } else { badge_count += *item as u64 - 96 }
                break;
            }
        }
    }
    badge_count
}