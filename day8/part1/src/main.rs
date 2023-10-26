use std::fs;

struct Trees {
    pub grid: Vec<Vec<usize>>,
}

impl From<&Vec<&str>> for Trees {
    fn from(value: &Vec<&str>) -> Self {
        let mut grid: Vec<Vec<usize>> = Vec::new();
        for line in value {
            let rows = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            grid.push(rows);
            grid.retain(|e| !e.is_empty());
        }
        Self { grid }
    }
}

impl Trees {
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let length = self.grid.len() - 1;
        let width = self.grid[0].len() - 1;
        let tree_height = self.grid[x][y];

        match (x, y) {
            (0, _) | (_, 0) => return true,
            (_, _) => {
                if x == length || y == width {
                    return true;
                }
            }
        }

        // the edge trees are visible by default
        let top: Vec<usize> = self.grid.iter().take(x).map(|v| v[y]).collect();
        let down: Vec<usize> = self.grid.iter().skip(x + 1).map(|v| v[y]).collect();
        let left: Vec<usize> = self.grid[x].iter().take(y).map(|&e| e).collect();
        let right: Vec<usize> = self.grid[x].iter().skip(y + 1).map(|&e| e).collect();

        let directions = [top, down, left, right];

        return directions
            .iter()
            .any(|v| v.iter().all(|&h| h < tree_height));
    }

    fn get_visible_count(&self) -> usize {
        let mut count = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if self.is_visible(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.split("\n").collect();

    let grid = Trees::from(&lines);

    println!(
        "The number of visible trees is {}",
        grid.get_visible_count()
    );
}
