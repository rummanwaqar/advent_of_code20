use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("input/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    let mut min_time = std::u64::MAX;
    let mut bus_id = 0;
    for option_bus in &input.bus_ids {
        match option_bus {
            Some(bus) => {
                let bus_time = ((input.start_time / bus) + 1) * bus;
                if bus_time < min_time {
                    min_time = bus_time;
                    bus_id = *bus;
                }
            }
            _ => {}
        }
    }
    (min_time - input.start_time) * bus_id
}

fn part2(input: &Input) -> u64 {
    // convert problem into a list of modulo equations
    // x === b (mod n)
    struct ModData {
        b: u64,
        n: u64,
    }
    let mut mods = Vec::new();
    for (i, n) in input.bus_ids.iter().enumerate() {
        if n.is_some() {
            let n = n.unwrap();
            // offsets are moved to other side and subtracted
            mods.push(ModData {
                b: (n - (i as u64) % n) % n,
                n,
            });
        }
    }

    // problem is solved using chinese remainder theorem
    let n: u64 = mods.iter().map(|x| x.n).product();
    let mut sum = 0;
    for m in mods.iter() {
        let b_i = m.b;
        let n_i = n / m.n;
        let x_i = modinverse::modinverse(n_i as i64, m.n as i64).unwrap();
        sum += b_i * n_i * x_i as u64;
    }
    sum % n
}

#[derive(Debug)]
struct Input {
    start_time: u64,
    bus_ids: Vec<Option<u64>>,
}

fn read_input(file_path: &str) -> std::io::Result<Input> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut lines = data.split('\n');
    let start_time: u64 = lines.next().unwrap().parse().unwrap();
    let mut bus_ids = Vec::new();
    for id in lines.next().unwrap().split(',') {
        if id != "x" {
            bus_ids.push(Some(id.parse().unwrap()));
        } else {
            bus_ids.push(None);
        }
    }
    Ok(Input {
        start_time,
        bus_ids,
    })
}
