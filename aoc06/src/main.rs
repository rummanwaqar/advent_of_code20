use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<String>) -> u32 {
    let mut groups: Vec<String> = Vec::new();
    let mut current_string: String = String::new();
    for line in input {
        if !line.is_empty() {
            current_string += &*line;
        } else {
            groups.push(current_string.clone());
            current_string.clear();
        }
    }
    groups.push(current_string.clone());

    let mut count = 0;
    for line in groups {
        let mut questions = HashSet::new();
        for c in line.chars() {
            questions.insert(c);
        }
        count += questions.len() as u32;
    }
    count
}

fn part2(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    let mut group: Vec<&str> = Vec::new();
    for line in input {
        if !line.is_empty() {
            group.push(line);
        } else {
            sum += n_common(&group);
            group.clear();
        }
    }
    sum += n_common(&group);
    sum
}

fn n_common(group: &Vec<&str>) -> u32 {
    let mut unique_set: HashSet<char> = group[0].chars().collect();
    for question in group.iter().skip(1) {
        unique_set = unique_set.intersection(&question.chars().collect()).map(|x| x.clone()).collect();
    }
    unique_set.len() as u32
}

fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let lines: Vec<String> = data.split('\n').map(|x| String::from(x)).collect();

    Ok(lines)
}