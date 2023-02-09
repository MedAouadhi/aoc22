use std::fs;

struct Game<'a> {
    play: (&'a str, &'a str),
}

impl<'a> Game<'a> {
    fn get_points(self) -> u32 {
        Game::get_outcome(self.play.0, self.play.1)
    }

    fn get_outcome(a: &str, b: &str) -> u32 {
        let mut out = 0;
        if b == "X" {
            match a {
                "A" => out = 3,
                "B" => out = 1,
                "C" => out = 2,
                &_ => todo!(),
            }
        } else if b == "Y" {
            match a {
                "A" => out = 1,
                "B" => out = 2,
                "C" => out = 3,
                &_ => todo!(),
            }
            out += 3;
        } else {
            match a {
                "A" => out = 2,
                "B" => out = 3,
                "C" => out = 1,
                &_ => todo!(),
            }
            out += 6;
        };
        out
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
