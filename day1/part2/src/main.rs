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

    let mut max: Vec<u32> = Vec::new();
    let mut sum = 0;
    for c in calories.iter() {
        if *c != 0 {
            sum += c;
            continue;
        }
        if max.len() < 3 {
            max.push(sum);
        } else {
            let min = *max.iter().rev().take(3).min().unwrap();
            if sum > min {
                let pos = max.iter().position(|&x| x == min).unwrap();
                max.remove(pos);
                max.insert(pos, sum);
            }
        }
        sum = 0;
    }
    sum = max.iter().rev().take(3).sum();
    println!("The max calories is = {}", sum);
}
