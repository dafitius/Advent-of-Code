use std::fs;
use anyhow::anyhow;

pub fn parse_input(path: &str) -> Option<Vec<u8>> {
    let data = fs::read_to_string(path).ok()?;
    Some(Vec::from(data.as_bytes()))
}

pub fn part1(input: &Vec<u8>) -> usize {
    if let Ok(answer) = get_marker(input, 4){
        answer
    }
    else{ 0 }
}

pub fn part2(input: &Vec<u8>) -> usize {
    if let Ok(answer) = get_marker(input, 14){
        answer
    }
    else{ 0 }
}


fn get_marker(input: &Vec<u8>, marker_size: usize) -> Result<usize, anyhow::Error>{

    if marker_size == 0{
        return Err(anyhow!("Can't find a marker sequence of length {}", marker_size))
    }

    let mut windows = input.windows(marker_size).collect::<Vec<_>>().into_iter();
    let marker_position_option = windows.position(|marker| !(1..marker.len()).any(|i| marker[i..].contains(&marker[i - 1])));

    return if let Some(marker_position) = marker_position_option {
        Ok(marker_position + marker_size)
    } else {
        Err(anyhow!("Couldn't find a marker sequence of length {}", marker_size))
    }
}