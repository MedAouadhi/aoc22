use std::fs;

struct Game<'a> {
    play: (&'a str, &'a str),
}

impl<'a> Game<'a> {
    fn get_points(self) -> u32 {
        let points = match self.play.1 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            &_ => todo!(),
        };
        match self.play {
            ("A", "X") => points + 3,
            ("A", "Y") => points + 6,
            ("A", "Z") => points,
            ("B", "X") => points,
            ("B", "Y") => points + 3,
            ("B", "Z") => points + 6,
            ("C", "X") => points + 6,
            ("C", "Y") => points,
            ("C", "Z") => points + 3,
            (&_, &_) => todo!(),
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Game<'a> {
    fn from(g: (&'a str, &'a str)) -> Self {
        Game { play: g }
    }
}

fn read_input<'a>(input: &'a str) -> Vec<(&'a str, &'a str)> {
    let mut output: Vec<(&str, &str)> = Vec::new();
    for line in input.clone().lines() {
        let play = line.split(' ').collect::<Vec<&str>>();
        let mut it = play.iter();
        output.push((*it.next().unwrap(), *it.next().unwrap()));
    }
    output
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Could not read input file");
    let play = read_input(input.as_str());
    let points: u32 = play.iter().map(|&x| Game::from(x).get_points()).sum();
    println!("The sum of points is {:?}", points);
}
