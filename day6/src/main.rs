use std::{collections::HashSet, fs};

fn is_unique(v: &[char]) -> bool {
    let hs: HashSet<_> = v.iter().collect();
    v.len() == hs.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read the file");

    let mut last_four: Vec<char> = Vec::new();
    //let create_count = 4; part1
    let create_count = 14;
    let mut index: Option<usize> = None;
    for (i, c) in input.chars().enumerate() {
        if last_four.len() == create_count - 1 {
            if !last_four.contains(&c) && is_unique(&last_four) {
                index = Some(i + 1);
                break;
            }
            last_four.remove(0);
        }
        last_four.push(c);
    }

    println!("The index is {:?}", index.unwrap());
}
