use std::fs;

fn read_input(path: &str) -> Vec<u32> {
    let input = fs::read_to_string(path).expect("Could not read input file");
    input
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or(0))
        .collect()
}

fn main() {
    let calories = read_input("input.txt");

    let mut max = 0;
    let mut sum = 0;
    for c in calories.iter() {
        if *c != 0 {
            sum += c;
            if sum > max {
                max = sum;
            }
            continue;
        }
        sum = 0;
    }
    println!("The max calories is = {}", max);
}
