use std::fs::File;
use std::io::Read;

type Map = Vec<Vec<char>>;
type Policy = fn(map: &Map, direction: &[i32; 2], x: usize, y: usize) -> bool;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", get_equilibrium_seats(&input, policy_adjacent, 4));
    println!("Part 2: {}", get_equilibrium_seats(&input, policy_first, 5));
}

fn get_equilibrium_seats(input: &Map, policy: Policy, empty_threshold: u64) -> u32 {
    let mut previous_map = input.to_owned();
    loop {
        let new_map = simulate_step(&previous_map, policy, empty_threshold);
        if new_map == previous_map {
            return count_occupied(&new_map);
        }
        previous_map = new_map;
    }
}

fn simulate_step(map: &Map, policy: Policy, empty_threshold: u64) -> Map {
    let mut new_map = map.clone();
    for y in 0..new_map.len() {
        for x in 0..new_map[0].len() {
            if map[y][x] == '.' {
                continue;
            }
            let neighbours = get_num_neighbours(&map, policy, x, y);
            if neighbours == 0 {
                new_map[y][x] = '#';
            } else if neighbours >= empty_threshold {
                new_map[y][x] = 'L';
            }
        }
    }
    new_map
}

fn get_num_neighbours(map: &Map, policy: Policy, x: usize, y: usize) -> u64 {
    let directions: [[i32; 2]; 8] = [
        [-1, -1],
        [0, -1],
        [1, -1],
        [-1, 0],
        [1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
    ];
    let mut sum = 0;
    for direction in directions.iter() {
        sum += policy(map, direction, x, y) as u64;
    }
    sum
}

fn policy_adjacent(map: &Map, direction: &[i32; 2], x: usize, y: usize) -> bool {
    let n_x: i32 = direction[0] + x as i32;
    let n_y: i32 = direction[1] + y as i32;
    in_map(n_x, n_y, map[0].len(), map.len()) && check(map, n_x, n_y, '#')
}

fn policy_first(map: &Map, direction: &[i32; 2], x: usize, y: usize) -> bool {
    let mut n_x: i32 = direction[0] + x as i32;
    let mut n_y: i32 = direction[1] + y as i32;
    while in_map(n_x, n_y, map[0].len(), map.len()) {
        if check(map, n_x, n_y, '#') {
            return true;
        } else if check(map, n_x, n_y, 'L') {
            return false;
        }
        n_x += direction[0];
        n_y += direction[1];
    }
    false
}

fn in_map(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

fn check(map: &Map, x: i32, y: i32, status: char) -> bool {
    map[y as usize][x as usize] == status
}

fn count_occupied(map: &Map) -> u32 {
    let mut num_seats = 0;
    for row in map {
        for seat in row {
            if *seat == '#' {
                num_seats += 1;
            }
        }
    }
    num_seats
}

fn read_input(file_path: &str) -> std::io::Result<Map> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(data
        .split('\n')
        .into_iter()
        .map(|x| x.chars().collect())
        .collect())
}
