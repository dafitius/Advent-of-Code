mod lib;

fn main() {
    if let Ok(lines) = lib::read_lines("./input/input.txt"){
        println!("The part 1 answer is: {}", lib::part1(lines));

    }

    //The files is being read twice. I know
    //I've got to figure out this borrow checker thing first 😅
    if let Ok(lines) = lib::read_lines("./input/input.txt"){
        println!("The part 2 answer is: {}", lib::part2(lines));
    }
}
