use std::fs::File;
use std::io::Read;
use std::collections::{VecDeque, HashMap};

fn main() {
    let input = read_input("input/input.txt").unwrap();
    let part1 = part1(&input, 25).unwrap();
    println!("part1: {}", part1);
    println!("part2: {}", part2(&input, part1).unwrap());
}

fn part1(input: &Vec<i64>, preamble_len: usize) -> Option<i64> {
    let mut queue = VecDeque::new();
    for x in input {
        if queue.len() < preamble_len {
            queue.push_back(*x);
        } else {
            if has_two_sum(queue.iter(), *x) {
                queue.pop_front();
                queue.push_back(*x);
            } else {
                return Some(*x);
            }
        }
    }
    None
}

fn part2(input: &Vec<i64>, search: i64) -> Option<i64> {
    // maintain two pointers
    // if current sum exceeds search moving the first pointer, else move second point
    let mut start = 0;
    let mut current_sum = input[0];

    for i in 1..input.len() {
        while current_sum > search && start < i - 1 {
            current_sum -= input[start];
            start += 1;
        }
        if current_sum == search {
            let sublist: Vec<i64> = (start..i).into_iter().map(|x| input[x]).collect();
            let min = sublist.iter().min().unwrap();
            let max = sublist.iter().max().unwrap();
            return Some(min + max);
        }
        current_sum += input[i];
    }
    None
}

fn has_two_sum<'a, I>(v: I, x: i64) -> bool
where I: Iterator<Item = &'a i64>
{
    // build hash map from two sum search
    let mut hash_map: HashMap<i64, i64> = HashMap::new();
    for num in v {
        let diff = x - num;
        if diff != *num {
            hash_map.insert(*num, diff);
        }
    }
    for (_, value) in &hash_map {
        if hash_map.contains_key(&value) {
            return true;
        }
    }
    false
}

fn read_input(file_path: &str) -> std::io::Result<Vec<i64>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let data = data.split('\n').map(|x| x.parse().unwrap()).collect();
    Ok(data)
}