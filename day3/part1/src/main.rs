use std::{collections::hash_set::HashSet, fs};

fn get_prios<'a>(input: &'a str) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    for line in input.clone().lines() {
        let play = line.split_at(line.len() / 2);
        let mut letters = HashSet::new();
        let _data: Vec<bool> = play
            .0
            .to_string()
            .as_bytes()
            .iter()
            .map(|&x| letters.insert(x))
            .collect();

        let prio: u8 = *play
            .1
            .to_string()
            .as_bytes()
            .iter()
            .filter(|&x| letters.contains(x))
            .collect::<Vec<&u8>>()[0];

        let ret_prio: u8 = match char::from(prio) {
            'a'..='z' => prio - 'a' as u8 + 1,
            'A'..='Z' => prio - 'A' as u8 + 26 + 1,
            _ => todo!(),
        };
        output.push(ret_prio as u32);
    }
    output
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Could not read input file");
    let total: u32 = get_prios(&input).iter().sum();
    println!("The total of priorities is {:?}", total);
}
