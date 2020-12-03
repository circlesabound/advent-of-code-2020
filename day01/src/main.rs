use std::io::{self, BufRead};

fn main() {
    // read lines
    let mut entries = vec![];
    for line in io::stdin().lock().lines() {
        let string = line.unwrap();
        let int = string.parse::<i32>().unwrap();
        entries.push(int);
    }

    for i in 0..entries.len() - 1 {
        for j in (i + 1)..entries.len() {
            if entries[i] + entries[j] == 2020 {
                println!("part 1: {}", entries[i] * entries[j]);
            }
        }
    }

    for i in 0..entries.len() - 2 {
        for j in (i + 1)..entries.len() - 1 {
            for k in (i + 2)..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    println!("part 2: {}", entries[i] * entries[j] * entries[k]);
                }
            }
        }
    }
}
