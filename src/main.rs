use regex::Regex;

fn day_3() {
    fn mul_instructions(contents: &str) -> i32 {
        let instruction_pattern: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let mut total = 0;
        for instruction in instruction_pattern.captures_iter(contents) {
            let a: i32 = instruction[1].parse().unwrap();
            let b: i32 = instruction[2].parse().unwrap();
            total += a * b;
        }

        total
    }

    //--------------------------------------------
    // Day 3
    //--------------------------------------------
    // Part 1
    // Match instruction pattern of the form "mul(1,2)" and extract the two numbers
    let contents = std::fs::read_to_string("inputs/3_1.txt").unwrap();
    let total = mul_instructions(&contents);
    println!("Total: {}", total);

    // Part 2
    let contents = std::fs::read_to_string("inputs/3_1.txt").unwrap();
    let instructions = contents
        .split("do()")
        .map(|item| item.split("don't()").collect::<Vec<_>>()[0])
        .collect::<Vec<_>>();

    let total = mul_instructions(&instructions.join(""));
    println!("Total: {}", total);
}

fn day_4() {
    // Part 1
    let contents = std::fs::read_to_string("inputs/4_1.txt").unwrap();
    let find_word: Vec<char> = "XMAS".chars().collect();

    println!("Find word: {:?}", find_word);

    let char_grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (n, m) = (char_grid[0].len(), char_grid.len());
    let l = find_word.len();

    println!("N: {}, M: {}, L: {}", n, m, l);

    let mut total_found = 0;
    for i in 0..n {
        for j in 0..m {
            if char_grid[i][j] == find_word[0]{
                if i <= n-l {
                    if find_word.iter().enumerate().map(|(k, c)| &char_grid[i+k][j] == c).reduce(|acc, e| acc&&e).unwrap(){
                        total_found += 1;
                    }
                    if j <= m-l {
                        if find_word.iter().enumerate().map(|(k, c)| &char_grid[i+k][j+k] == c).reduce(|acc, e| acc&&e).unwrap(){
                            total_found += 1;
                        }
                    }
                    if j >= l-1 {
                        if find_word.iter().enumerate().map(|(k, c)| &char_grid[i+k][j-k] == c).reduce(|acc, e| acc&&e).unwrap(){
                            total_found += 1;
                        }
                    }
                }
                if i >= l-1 {
                    if find_word.iter().enumerate().map(|(k, c)| &char_grid[i-k][j] == c).reduce(|acc, e| acc&&e).unwrap(){
                        total_found += 1;
                    }
                    if j <= m-l {
                        if find_word.iter().enumerate().map(|(k, c)| &char_grid[i-k][j+k] == c).reduce(|acc, e| acc&&e).unwrap(){
                            total_found += 1;
                        }
                    }
                    if j >= l-1 {
                        if find_word.iter().enumerate().map(|(k, c)| &char_grid[i-k][j-k] == c).reduce(|acc, e| acc&&e).unwrap(){
                            total_found += 1;
                        }
                    }
                }

                if j <= m-l {
                    if find_word.iter().enumerate().map(|(k, c)| &char_grid[i][j+k] == c).reduce(|acc, e| acc&&e).unwrap(){
                        total_found += 1;
                    }
                }
                if j >= l-1 {
                    if find_word.iter().enumerate().map(|(k, c)| &char_grid[i][j-k] == c).reduce(|acc, e| acc&&e).unwrap(){
                        total_found += 1;
                    }
                }
            }
        }
    }

    println!("Part 1 total found: {}", total_found);

    let find_word: Vec<char> = "MAS".chars().collect();

    let mut total_found = 0;
    for i in 1..n-1 {
        for j in 1..m-1 {
            if char_grid[i][j] == find_word[1]{
                // Check for mas on the diagonals
                let x1_f= find_word.iter().enumerate().map(|(k, c)| &char_grid[i-1+k][j-1+k] == c).reduce(|acc, e| acc&&e).unwrap();
                let x1_b= find_word.iter().enumerate().map(|(k, c)| &char_grid[i+1-k][j+1-k] == c).reduce(|acc, e| acc&&e).unwrap();

                let x2_f= find_word.iter().enumerate().map(|(k, c)| &char_grid[i+1-k][j-1+k] == c).reduce(|acc, e| acc&&e).unwrap();
                let x2_b= find_word.iter().enumerate().map(|(k, c)| &char_grid[i-1+k][j+1-k] == c).reduce(|acc, e| acc&&e).unwrap();

                if (x1_f || x1_b) && (x2_f || x2_b){
                    total_found += 1;
                }
            }
        }
    }

    println!("Part 2 total found: {}", total_found);


}

fn main() {
    // day_3();

    day_4();
}
