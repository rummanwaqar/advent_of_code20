use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(bags: &Vec<Bag>) -> u32 {
    let mut final_bags: HashSet<&str> = HashSet::new();
    // since process_queue does not guarantee uniqueness we might process the same
    // bag multiple times. final_bags is a set and it will remove any duplicates
    let mut process_queue: VecDeque<&str> = VecDeque::from(vec!["shiny gold"]);
    while !process_queue.is_empty() {
        let key = process_queue.pop_front().unwrap();
        for bag in bags {
            if bag.contents.contains_key(key) {
                process_queue.push_back(&*bag.name);
            }
        }
        final_bags.insert(key);
    }
    // minus 1 to remove the original bag (shiny gold)
    final_bags.len() as u32 - 1
}

fn part2(bags: &Vec<Bag>) -> u32 {
    // convert vector to hashmap for quick look up
    let mut bag_map: HashMap<&str, &Bag> = HashMap::new();
    for bag in bags {
        bag_map.insert(&*bag.name, bag);
    }
    // don't count the shiny gold bag
    get_size_contents(&bag_map, "shiny gold") - 1
}

fn get_size_contents(map: &HashMap<&str, &Bag>, bag_name: &str) -> u32 {
    let bag = map.get(bag_name).unwrap().to_owned();
    let mut size = 0;
    for content in &bag.contents {
        size += content.1 * get_size_contents(&map, &*content.0);
    }
    size + 1
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Bag>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let bags: Vec<Bag> = data.split('\n').map(|x| x.parse().unwrap()).collect();
    Ok(bags)
}

#[derive(Debug)]
struct Bag {
    name: String,
    contents: HashMap<String, u32>,
}

impl FromStr for Bag {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // remove period and all instances of bag/bags
        let new_line = line[..line.len() - 1]
            .replace("bags", "")
            .replace("bag", "");
        // split over contain
        let name_content_split: Vec<&str> = new_line.split("contain").collect();
        let mut bag = Bag {
            name: name_content_split[0].trim().to_string(),
            contents: HashMap::new(),
        };
        // check if bag contains other bags
        if name_content_split[1].trim() != "no other" {
            let content_split: Vec<&str> = name_content_split[1].split(',').collect();
            for content in content_split {
                let qty_name_split: Vec<&str> = content.trim().split(' ').collect();
                let quantity: u32 = qty_name_split[0].parse().unwrap();
                // assuming that all names have two words
                let content_name: String =
                    qty_name_split[1].trim().to_string() + " " + qty_name_split[2].trim();
                bag.contents.insert(content_name, quantity);
            }
        }
        Ok(bag)
    }
}
