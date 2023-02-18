#![feature(iter_array_chunks)]
use std::fs;

fn extract_op(cmd: &str) -> (u32, u32, u32) {
    let numbers: Vec<u32> = cmd
        .split(' ')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();
    (numbers[0], numbers[1], numbers[2])
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("stacks.txt not found");
    // extract the first 8 lines from 'stacks'
    let drawing = input.lines().take(8).collect::<Vec<&str>>();

    // initialize an array of 8 vectors of strings
    let mut stack: Vec<Vec<String>> = vec![vec![]; 9];

    for line in drawing {
        for (index, element) in line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|e| e.iter().collect::<String>())
            .enumerate()
        {
            // We omit the empty elements, and we push to the front of the stack since we are reading from the back of the
            //stacks in the drawings so that we can use normal push and pop later.
            if !element.trim().is_empty() {
                stack[index].insert(0, element.as_str().trim().to_string());
            }
        }
    }

    // apply the operations
    for operation in input.lines().skip(10) {
        let (cnt, from, to) = extract_op(operation);
        for _ in 0..cnt {
            let val = stack[from as usize - 1].pop().unwrap();
            stack[to as usize - 1].push(val);
        }
    }

    println!(
        " The final answer is: {}",
        stack
            .iter()
            .map(|v| v
                .last()
                .unwrap_or(&"".to_string())
                .trim_matches(|c| c == '[' || c == ']')
                .to_owned())
            .collect::<String>()
    );
}
