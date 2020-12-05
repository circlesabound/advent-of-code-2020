use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passes = Pass::read_all_from_stdin()?;
    let ids: Vec<u32> = passes.into_iter().map(|p| p.id()).collect();
    let max_id = ids.iter().max().expect("asdf");
    println!("part 1: {}", max_id);

    let min_id = ids.iter().min().expect("asdf");
    let sum: u32 = (min_id.clone()..=max_id.clone()).sum();
    let my_sum: u32 = ids.iter().sum();
    println!("part 2: {}", sum - my_sum);
    Ok(())
}

struct Pass {
    row: u32,
    col: u32,
}

impl Pass {
    fn read_all_from_stdin() -> Result<Vec<Pass>, io::Error> {
        io::stdin()
            .lock()
            .lines()
            .map(|l| Pass::from_string(l?))
            .collect()
    }

    fn from_string(string: String) -> Result<Pass, io::Error> {
        if string.len() != 10 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expect 10 characters in a pass",
            ))
        } else {
            let (row_str, col_str) = string.split_at(7);
            let row = row_str
                .chars()
                .try_fold((0, 127), |(min, max), ch| match ch {
                    'F' => Ok(get_lower_half(min, max)),
                    'B' => Ok(get_upper_half(min, max)),
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "bad character")),
                })?
                .0;

            let col = col_str
                .chars()
                .try_fold((0, 8), |(min, max), ch| match ch {
                    'L' => Ok(get_lower_half(min, max)),
                    'R' => Ok(get_upper_half(min, max)),
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "bad character")),
                })?
                .0;

            Ok(Pass { row, col })
        }
    }

    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

fn get_lower_half(min: u32, max: u32) -> (u32, u32) {
    let new_upper_bound = max - ((max - min + 1) / 2);
    (min, new_upper_bound)
}

fn get_upper_half(min: u32, max: u32) -> (u32, u32) {
    let new_lower_bound = min + ((max - min + 1) / 2);
    (new_lower_bound, max)
}
