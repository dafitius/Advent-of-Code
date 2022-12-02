use std::fs;

pub fn parse_input(path: &str) -> Option<Vec<Vec<char>>> {
    let data = fs::read_to_string(path).ok()?;

    let lines: Vec<Vec<char>> = data.split("\n").map(|i| {
        i.split(' ').map(|i| i.trim_end().parse().unwrap()).collect()
    }).collect();

    Some(lines)
}

pub fn part1(matches: &Vec<Vec<char>>) -> u64 {
    let mut points: u64 = 0;
    for rps_match in matches {
        let opponent = rps_match[0] as i16 - 64;
        let guide = rps_match[1] as i16 - 87;

        points += guide as u64;

        if guide == opponent {
            points += 3;
            continue;
        }

        if guide - (opponent % 3) == 1 {
            points += 6;
        }
    }
    points
}

pub fn part2(matches: &Vec<Vec<char>>) -> u64 {
    let mut points: u64 = 0;

    for rps_match in matches {
        let opponent = rps_match[0] as i16 - ('A' as i16  -1);
        let guide = rps_match[1] as i16 - ('X' as i16 -1);

        match guide {
            1 => {
                let mut rot = vec![1, 2, 3];
                rot.rotate_right(1);
                points += rot[(opponent-1) as usize];
                continue;
            },
            2 =>{
                points += opponent as u64;
                points += 3;
                continue;
            },
            3 => {
                let mut rot = vec![1, 2, 3];
                rot.rotate_left(1);
                points += rot[(opponent - 1) as usize];
                points += 6;
                continue;
            }
            _ =>{}
        }
    }
    points
}