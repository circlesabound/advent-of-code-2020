use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::read_from_stdin()?;
    
    println!("part 1: {}", simulate_slope(&map, 3, 1));
    println!(
        "part 2: {}",
        simulate_slope(&map, 1, 1) *
        simulate_slope(&map, 3, 1) *
        simulate_slope(&map, 5, 1) *
        simulate_slope(&map, 7, 1) *
        simulate_slope(&map, 1, 2),
    );

    Ok(())
}

fn simulate_slope(map: &Map, x_shift: usize, y_shift: usize) -> i64 {
    let mut tree_count = 0;

    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < map.bottom() {
        // check position
        if let Tile::Tree = map.get(&pos_x, &pos_y).expect("uh oh") {
            tree_count += 1;
        }

        // shift
        pos_x += x_shift;
        pos_y += y_shift;
    }

    tree_count
}

#[derive(Clone, Copy)]
enum Tile {
    Open,
    Tree,
}

struct Map {
    grid: Vec<Vec<Tile>>
}

impl Map {
    fn bottom(&self) -> usize {
        self.grid.len()
    }

    fn get(&self, x: &usize, y: &usize) -> Option<Tile> {
        let x_mod = x % self.grid[0].len();
        Some(*self.grid.get(*y)?.get(x_mod)?)
    }

    fn read_from_stdin() -> Result<Map, io::Error> {
        let mut line_length = None;
        let mut grid = vec![];
        for line in io::stdin().lock().lines() {
            let line = line?;
            let chars = line.chars();
            let mut row = vec![];
            for c in chars {
                match c {
                    '.' => row.push(Tile::Open),
                    '#' => row.push(Tile::Tree),
                    other => return Err(io::Error::new(
                        io::ErrorKind::InvalidData, 
                        format!("Expected '.' or '#' in input data, got {}", other))),
                }
            }

            if line_length.is_none() {
                line_length.replace(row.len());
            } else if line_length.unwrap() != row.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "input data rows vary in length",
                ));
            }

            grid.push(row);
        }

        Ok(Map {
            grid,
        })
    }
}
