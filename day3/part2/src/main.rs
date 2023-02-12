#![feature(iter_array_chunks)]
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    rc::Rc,
};

fn get_prios<'a>(input: &'a str) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();

    for line in input.clone().lines().array_chunks::<3>() {
        let letters = Rc::new(RefCell::new(HashMap::new()));
        let _data: Vec<_> = line
            .iter()
            .map(|&x| x.as_bytes())
            .collect::<Vec<_>>()
            .iter()
            .map(|&x| {
                x.iter()
                    .cloned()
                    .collect::<HashSet<u8>>()
                    .iter()
                    .map(|&l| {
                        let value = *letters.borrow().get(&l).unwrap_or(&0);
                        letters.borrow_mut().insert(l, value + 1);
                    })
                    .for_each(drop);
            })
            .collect();
        let item: u8 = *letters
            .borrow()
            .keys()
            .filter(|&x| letters.borrow()[x] == 3)
            .collect::<Vec<_>>()[0];

        let ret_prio: u8 = match char::from(item) {
            'a'..='z' => item - 'a' as u8 + 1,
            'A'..='Z' => item - 'A' as u8 + 26 + 1,
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
