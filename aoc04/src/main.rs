use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}


fn part1(passports: &Vec<Passport>) -> u32 {
    let mut valid = 0;
    for passport in passports {
        valid += (!passport.byr.is_empty()
            && !passport.iyr.is_empty()
            && !passport.eyr.is_empty()
            && !passport.hgt.is_empty()
            && !passport.hcl.is_empty()
            && !passport.ecl.is_empty()
            && !passport.pid.is_empty()) as u32;
    }
    valid
}

fn part2(passports: &Vec<Passport>) -> u32 {
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    cid (Country ID) - ignored, missing or not.
     */
    let mut valid = 0;
    for passport in passports {
        let is_valid = is_year_valid(&*passport.byr, 1920, 2002)
            && is_year_valid(&*passport.iyr, 2010, 2020)
            && is_year_valid(&*passport.eyr, 2020, 2030)
            && is_height_valid(&*passport.hgt)
            && is_hair_color_valid(&*passport.hcl)
            && is_eye_color_valid(&*passport.ecl)
            && is_passport_id_valid(&*passport.pid);
        valid += is_valid as u32;
    }
    valid
}

fn is_year_valid(string: &str, min: u32, max: u32) -> bool {
    if string.len() == 4 {
        return match string.parse::<u32>() {
            Ok(number) => number >= min && number <= max,
            Err(_) => false,
        };
    }
    false
}

fn is_height_valid(string: &str) -> bool {
    /*
     hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
     */
    let length = string.len();
    if length >= 4 {
        let number = &string[..length-2];

        return match number.parse::<u32>() {
            Ok(number) => {
                let unit = &string[length-2..length];
                return match unit {
                    "in" => number >= 59 && number <= 76,
                    "cm" => number >= 150 && number <= 193,
                    _ => false
                };
            },
            Err(_) => false,
        };
    }
    false
}

fn is_hair_color_valid(string: &str) -> bool {
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if string.len() == 7 && string.chars().nth(0).unwrap() == '#' {
        for c in string[1..].bytes() {
            if c < 48 || (c >= 58 && c <= 96) || c > 102 {
                return false;
            }
        }
        return true;
    }
    false
}

fn is_eye_color_valid(string: &str) -> bool {
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth
    let colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
    string.len() == 3 && colors.contains(string)
}

fn is_passport_id_valid(string: &str) -> bool {
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    return string.len() == 9 && string.parse::<u32>().is_ok()
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Passport>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut passports: Vec<Passport> = Vec::new();
    let lines: Vec<&str> = data.split('\n').collect();

    let mut vec: Vec<&str> = Vec::new();
    for line in lines {
        let fields: Vec<&str> = line.trim().split(' ').collect();
        if fields.len() == 0 || (fields.len() == 1 && fields[0].is_empty()) {
            passports.push(Passport::from(&vec));
            vec.clear();
        } else {
            vec.extend(fields);
        }
    }
    passports.push(Passport::from(&vec));

    Ok(passports)
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    pub fn from(vec: &Vec<&str>) -> Self {
        let mut passport: Passport = Passport {
            byr: "".to_string(),
            iyr: "".to_string(),
            eyr: "".to_string(),
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
            cid: "".to_string(),
        };
        for field in vec {
            let key_val: Vec<&str> = field.trim().split(':').collect();
            if key_val.len() == 2 {
                match key_val[0] {
                    "byr" => passport.byr = String::from(key_val[1]),
                    "iyr" => passport.iyr = String::from(key_val[1]),
                    "eyr" => passport.eyr = String::from(key_val[1]),
                    "hgt" => passport.hgt = String::from(key_val[1]),
                    "hcl" => passport.hcl = String::from(key_val[1]),
                    "ecl" => passport.ecl = String::from(key_val[1]),
                    "pid" => passport.pid = String::from(key_val[1]),
                    "cid" => passport.cid = String::from(key_val[1]),
                    _ => {
                        println!("Invalid field in passport: {:?}", key_val);
                    }
                }
            }
        }
        passport
    }
}
