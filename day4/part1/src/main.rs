use std::fs;

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
            (elf1[0] >= elf2[0] && elf1[1] <= elf2[1]) || (elf2[0] >= elf1[0] && elf2[1] <= elf1[1])
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
