use regex::Regex;

fn main() {

    // Part 1
    // Match instruction pattern of the form "mul(1,2)" and extract the two numbers
    let contents = std::fs::read_to_string("inputs/3_1.txt").unwrap();
    let instruction_pattern: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for instruction in instruction_pattern.captures_iter(&contents) {
        let a: i32 = instruction[1].parse().unwrap();
        let b: i32 = instruction[2].parse().unwrap();
        total += a * b;
    }
    println!("Total: {}", total);

    // Part 2
    let mut contents = std::fs::read_to_string("inputs/3_1.txt").unwrap();

    let instructions = contents.split("do()").map(|item| item.split("don't()").collect::<Vec<_>>()[0]).collect::<Vec<_>>();

    let mut total = 0;
    for instruction in instruction_pattern.captures_iter(&instructions.join("")) {
        let a: i32 = instruction[1].parse().unwrap();
        let b: i32 = instruction[2].parse().unwrap();
        total += a * b;
    }
    println!("Total: {}", total);

}
