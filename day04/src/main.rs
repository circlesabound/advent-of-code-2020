use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = read_from_stdin()?;

    println!("part 2: {}", res.iter().filter(|p| p.is_valid()).count());
    Ok(())
}

fn read_from_stdin() -> Result<Vec<Passport>, io::Error> {
    let mut passports = vec![];
    let mut current_passport = Some(Passport::new());
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            // empty line, end of passport
            passports.push(current_passport.replace(Passport::new()).unwrap());
            continue;
        }

        for field_str in line.split_whitespace() {
            match PassportField::try_from(field_str) {
                Some(field) => {
                    current_passport.as_mut().unwrap().add_field(field);
                },
                None => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "bad passport field",
                )),
            }
        }
    }

    passports.push(current_passport.take().unwrap());

    Ok(passports)
}



struct Passport {
    birth_year: bool,
    issue_year: bool,
    expiration_year: bool,
    height: bool,
    hair_colour: bool,
    eye_colour: bool,
    passport_id: bool,
    country_id: bool,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: false,
            issue_year: false,
            expiration_year: false,
            height: false,
            hair_colour: false,
            eye_colour: false,
            passport_id: false,
            country_id: false,
        }
    }

    fn add_field(&mut self, field: PassportField) {
        match field {
            PassportField::BirthYear(val) => {
                if let Ok(yr) = val.parse::<u32>() {
                    self.birth_year = (1920..=2002).contains(&yr);
                }
            }
            PassportField::IssueYear(val) => {
                if let Ok(yr) = val.parse::<u32>() {
                    self.issue_year = (2010..=2020).contains(&yr);
                }
            }
            PassportField::ExpirationYear(val) => {
                if let Ok(yr) = val.parse::<u32>() {
                    self.expiration_year = (2020..=2030).contains(&yr);
                }
            }
            PassportField::Height(val) => {
                if let Some(cm) = val.strip_suffix("cm") {
                    if let Ok(cm) = cm.parse::<u32>() {
                        self.height = (150..=193).contains(&cm);
                    }
                } else if let Some(inch) = val.strip_suffix("in") {
                    if let Ok(inch) = inch.parse::<u32>() {
                        self.height = (59..=76).contains(&inch);
                    }
                }
            }
            PassportField::HairColour(val) => {
                let hair_colour_chars = vec![
                    '0',
                    '1',
                    '2',
                    '3',
                    '4',
                    '5',
                    '6',
                    '7',
                    '8',
                    '9',
                    'a',
                    'b',
                    'c',
                    'd',
                    'e',
                    'f',
                ];
                if val.len() == 7 {
                    if let Some(val) = val.strip_prefix('#') {
                        self.hair_colour = val.chars().all(|c| hair_colour_chars.contains(&c));
                    }
                }
            }
            PassportField::EyeColour(val) => {
                let eye_colours = vec![
                    "amb",
                    "blu",
                    "brn",
                    "gry",
                    "grn",
                    "hzl",
                    "oth"
                ];
                self.eye_colour = eye_colours.contains(&val.as_str());
            }
            PassportField::PassportId(val) => {
                self.passport_id = val.len() == 9 && val.parse::<u32>().is_ok()
            }
            PassportField::CountryId(val) => self.country_id = true,
        }
    }

    fn is_valid(&self) -> bool {
        self.birth_year &&
        self.issue_year &&
        self.expiration_year &&
        self.height &&
        self.hair_colour &&
        self.eye_colour &&
        self.passport_id
    }
}

enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColour(String),
    EyeColour(String),
    PassportId(String),
    CountryId(String),
}

impl PassportField {
    fn try_from(input: &str) -> Option<PassportField> {
        let spl = input.split_at(3);
        match spl.0 {
            "byr" => Some(PassportField::BirthYear(spl.1.trim_start_matches(':').to_owned())),
            "iyr" => Some(PassportField::IssueYear(spl.1.trim_start_matches(':').to_owned())),
            "eyr" => Some(PassportField::ExpirationYear(spl.1.trim_start_matches(':').to_owned())),
            "hgt" => Some(PassportField::Height(spl.1.trim_start_matches(':').to_owned())),
            "hcl" => Some(PassportField::HairColour(spl.1.trim_start_matches(':').to_owned())),
            "ecl" => Some(PassportField::EyeColour(spl.1.trim_start_matches(':').to_owned())),
            "pid" => Some(PassportField::PassportId(spl.1.trim_start_matches(':').to_owned())),
            "cid" => Some(PassportField::CountryId(spl.1.trim_start_matches(':').to_owned())),
            _ => None,
        }
    }
}