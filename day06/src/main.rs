use std::{
    collections::HashMap,
    io::{self, BufRead},
    str::Chars,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let groups = read_from_stdin()?;
    println!(
        "part 1: {}",
        groups.iter().fold(0, |a, g| a + g.count_part1())
    );
    println!(
        "part 2: {}",
        groups.iter().fold(0, |a, g| a + g.count_part2())
    );

    Ok(())
}

fn read_from_stdin() -> Result<Vec<Group>, io::Error> {
    let mut groups = vec![];
    let mut current_group = Some(Group::new());
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            // empty line, end of group
            groups.push(current_group.replace(Group::new()).unwrap());
            continue;
        }

        current_group.as_mut().unwrap().add_responses(line.chars());
    }

    groups.push(current_group.take().unwrap());

    Ok(groups)
}

struct Group {
    answers: HashMap<char, u32>,
    people: u32,
}

impl Group {
    fn new() -> Group {
        let answers = ('a'..='z').map(|c| (c, 0)).collect();
        Group { answers, people: 0 }
    }

    fn add_responses(&mut self, chars: Chars) {
        for c in chars {
            self.answers.entry(c).and_modify(|v| *v += 1);
        }
        self.people += 1;
    }

    fn count_part1(&self) -> usize {
        self.answers.values().filter(|b| **b > 0).count()
    }

    fn count_part2(&self) -> usize {
        self.answers.values().filter(|b| **b == self.people).count()
    }
}
