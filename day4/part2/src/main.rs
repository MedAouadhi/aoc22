use std::fs;

// a function that accepts a vector of strings, returns a vector of integers converted from the first part of the splitted element of the input vector
fn is_contained(a: Vec<&str>) -> bool {
    let limits = a
        .iter()
        .map(|x| {
            x.split('-')
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    match limits.as_slice() {
        [elf1, elf2] => {
            let min = elf1[0].min(elf2[0]);
            // We verify that the minimum value of one range is between the values of the other range,
            // this way we are sure that the ranges overlap.
            elf1[0] == min && elf1[1] >= elf2[0] || elf2[0] == min && elf2[1] >= elf1[0]
        }
        &[] | &[_] | &[_, _, _, ..] => unreachable!(),
    }
}

fn main() {
    let pairs = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(',').collect::<Vec<&str>>())
        .filter(|e| is_contained(e.to_vec()))
        .count();

    println!("The count of pairs is {:#?}", pairs);
}
