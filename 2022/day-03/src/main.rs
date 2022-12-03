mod lib;

fn main() {
    if let Some(matches) = lib::parse_input("input/input.txt") {
        println!("The answer to part 1 is: {}", lib::part1(&matches));
        println!("The answer to part 2 is: {}", lib::part2(&matches));
    }
}
