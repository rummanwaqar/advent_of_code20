use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    let mut error_rate = 0;
    for ticket in input.other_tickets.iter() {
        for n in ticket {
            let mut is_valid = false;
            for field in input.fields.iter() {
                if field.valid_input(*n) {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                error_rate += n;
            }
        }
    }
    error_rate
}

fn part2(input: &Input) -> u64 {
    let mut valid_classifications: Vec<Vec<Vec<String>>> = Vec::new();
    for ticket in input.other_tickets.iter() {
        let classification = classify_ticket(ticket, &input.fields);
        if is_ticket_valid(&classification) {
            valid_classifications.push(classification);
        }
    }

    // classify fields
    let mut final_classifications = Vec::new();
    for f in 0..input.fields.len() {
        let classes: Vec<Vec<String>> = valid_classifications
            .iter()
            .map(|x| x.iter().nth(f).unwrap().clone())
            .collect();
        final_classifications.push(simplify_class(&classes));
    }
    // get final field name to index correlation
    let mut field_names: Vec<String> = vec!["".to_string(); input.fields.len()];
    while field_names.iter().any(|x| x.is_empty()) {
        // find index with len = 1
        let index = final_classifications
            .iter()
            .position(|x| x.len() == 1)
            .unwrap();
        let value = final_classifications
            .iter()
            .nth(index)
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone();
        // add that field to field names
        field_names[index] = value.clone();
        // remove that field from all fields
        for xs in &mut final_classifications {
            xs.retain(|x| x.as_str() != value);
        }
    }

    let mut answer = 1;
    for (index, field) in field_names.iter().enumerate() {
        if field.contains("departure") {
            answer *= input.my_ticket[index];
        }
    }
    answer
}

fn classify_ticket(ticket: &Vec<u64>, fields: &Vec<Field>) -> Vec<Vec<String>> {
    let mut classification = Vec::new();
    for field in ticket {
        let mut field_class: Vec<String> = Vec::new();
        for known_field in fields {
            if known_field.valid_input(*field) {
                field_class.push((*known_field.name).parse().unwrap());
            }
        }
        classification.push(field_class);
    }
    classification
}

fn simplify_class(classes: &Vec<Vec<String>>) -> Vec<String> {
    let mut count_map = HashMap::new();
    for x in classes {
        for y in x {
            *count_map.entry(y.as_str()).or_insert(0) += 1;
        }
    }
    let common: Vec<String> = count_map
        .iter()
        .filter(|(_x, &y)| y == classes.len())
        .map(|(x, _y)| x.to_string())
        .collect();
    common
}

fn is_ticket_valid(classification: &Vec<Vec<String>>) -> bool {
    for field in classification {
        if field.is_empty() {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct Field {
    name: String,
    range1: Range<u64>,
    range2: Range<u64>,
}

impl Field {
    fn valid_input(self: &Self, n: u64) -> bool {
        self.range1.contains(&n) || self.range2.contains(&n)
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // class: 1-3 or 5-7

        let mut name_range_split = s.split(": ");
        let name = name_range_split.next().unwrap().to_string();

        let mut range_split = name_range_split.next().unwrap().split(" or ");

        let range1 = str_to_range(range_split.next().unwrap());
        let range2 = str_to_range(range_split.next().unwrap());

        Ok(Self {
            name,
            range1,
            range2,
        })
    }
}

fn str_to_range(s: &str) -> Range<u64> {
    let start = s.split('-').nth(0).unwrap().parse().unwrap();
    let end: u64 = s.split('-').nth(1).unwrap().parse().unwrap();
    Range {
        start,
        end: end + 1,
    }
}

#[derive(Debug)]
struct Input {
    fields: Vec<Field>,
    my_ticket: Vec<u64>,
    other_tickets: Vec<Vec<u64>>,
}

fn read_input(file_path: &str) -> std::io::Result<Input> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut data = data.split("\n\n");

    let mut fields = Vec::new();
    for field_str in data.next().unwrap().split('\n') {
        fields.push(Field::from_str(field_str).unwrap());
    }

    let my_ticket: Vec<u64> = data
        .next()
        .unwrap()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut other_tickets = Vec::new();
    for ticket_string in data.next().unwrap().split('\n').skip(1) {
        let ticket: Vec<u64> = ticket_string
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        other_tickets.push(ticket);
    }

    Ok(Input {
        fields,
        my_ticket,
        other_tickets,
    })
}
