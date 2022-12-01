use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;


fn parse_elves(lines: Lines<io::BufReader<File>>) -> Vec<Vec<u64>> {
    let mut elves: Vec<Vec<u64>> = vec![];
    let mut index: usize = 0;

    elves.push(vec![]);
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                index += 1;
                elves.push(vec![]);
            }

            if let Ok(calories) = ip.parse() {
                elves[index].push(calories);
            }
        }
    }
    elves
}

pub fn part1(lines: Lines<io::BufReader<File>>) -> u64 {
    parse_elves(lines).iter().map(|elf| elf.iter().sum()).max().unwrap_or(0)
}

pub fn part2(lines: Lines<io::BufReader<File>>) -> u64 {
    let mut top: Vec<u64> = vec![0, 0, 0];

    for elf_calorie_sum in parse_elves(lines).iter().map(|elf| elf.iter().sum::<u64>()) {
        for value in top.iter() {
            if elf_calorie_sum >= *value {

                //insert the newly found value inside the array
                top.insert(0, elf_calorie_sum);

                //remove the lowest value inside the array
                top.remove(top.iter().position(|element| element == top.iter().min().unwrap()).unwrap());

                break;
            }
        }
    }
    top.iter().sum()
}


pub fn read_lines<P>(filename: P) -> io::Result<Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}