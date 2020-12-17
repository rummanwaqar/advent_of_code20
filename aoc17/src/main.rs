use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    // let input = vec![
    //     vec!['.', '#', '.'],
    //     vec!['.', '.', '#'],
    //     vec!['#', '#', '#'],
    // ];
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<Vec<char>>) -> u64 {
    let mut on = HashSet::new();
    for (r, row_vec) in input.iter().enumerate() {
        for (c, &ch) in row_vec.iter().enumerate() {
            if ch == '#' {
                on.insert((r as i32, c as i32, 0 as i32));
            }
        }
    }
    let input_size = input.len() as i32;

    let n_cycles = 6;
    for i in 1..n_cycles+1 {
        let mut new_on = HashSet::new();
        for x in (-input_size - i)..(input_size + i + 1) {
            for y in (-input_size - i)..(input_size + i + 1) {
                for z in -i..(i + 1) {
                    let n_neighbours = count_neighbours_3d(&(x, y, z), &on);
                    if on.contains(&(x, y, z)) {
                        if n_neighbours == 2 || n_neighbours == 3 {
                            new_on.insert((x, y, z));
                        }
                    } else {
                        if n_neighbours == 3 {
                            new_on.insert((x, y, z));
                        }
                    }
                }
            }
        }
        on = new_on;
    }
    on.len() as u64
}

fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut on = HashSet::new();
    for (r, row_vec) in input.iter().enumerate() {
        for (c, &ch) in row_vec.iter().enumerate() {
            if ch == '#' {
                on.insert((r as i32, c as i32, 0 as i32, 0 as i32));
            }
        }
    }
    let input_size = input.len() as i32;
    let n_cycles = 6;
    for i in 1..n_cycles+1 {
        let mut new_on = HashSet::new();
        for x in (-input_size - i)..(input_size + i + 1) {
            for y in (-input_size - i)..(input_size + i + 1) {
                for z in -i..(i + 1) {
                    for w in -i..(i + 1) {
                        let n_neighbours = count_neighbours_4d(&(x, y, z, w), &on);
                        if on.contains(&(x, y, z, w)) {
                            if n_neighbours == 2 || n_neighbours == 3 {
                                new_on.insert((x, y, z, w));
                            }
                        } else {
                            if n_neighbours == 3 {
                                new_on.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        on = new_on;
    }
    on.len() as u64
}

fn count_neighbours_3d(point: &(i32, i32, i32), on: &HashSet<(i32, i32, i32)>) -> u32 {
    let offsets: [i32; 3] = [-1, 0, 1];
    let mut n_neighbours = 0;
    for &dx in offsets.iter() {
        for &dy in offsets.iter() {
            for &dz in offsets.iter() {
                if !(dx == 0 && dy == 0 && dz == 0)
                    && on.contains(&(point.0 + dx, point.1 + dy, point.2 + dz))
                {
                    n_neighbours += 1;
                }
            }
        }
    }
    n_neighbours
}

fn count_neighbours_4d(point: &(i32, i32, i32, i32), on: &HashSet<(i32, i32, i32, i32)>) -> u32 {
    let offsets: [i32; 3] = [-1, 0, 1];
    let mut n_neighbours = 0;
    for &dx in offsets.iter() {
        for &dy in offsets.iter() {
            for &dz in offsets.iter() {
                for &dw in offsets.iter() {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0)
                        && on.contains(&(point.0 + dx, point.1 + dy, point.2 + dz, point.3 + dw))
                    {
                        n_neighbours += 1;
                    }
                }
            }
        }
    }
    n_neighbours
}

fn read_input(file_path: &str) -> std::io::Result<Vec<Vec<char>>> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut input = Vec::new();
    for row in data.split('\n') {
        input.push(row.chars().collect::<Vec<char>>());
    }
    Ok(input)
}
