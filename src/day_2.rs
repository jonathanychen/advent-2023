use std::{fs, collections::HashMap, cmp};

pub fn part_1() -> i128 {
    let contents = fs::read_to_string("inputs/day_2.txt")
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');

    let mut sum: i128 = 0;

    let maxes: HashMap<&str, i128> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in lines {
        println!("{line}");

        let mut tokens = line.split(' ');
        tokens.next();
        let id_str = tokens.next().expect("Expected string").trim_matches(':');
        let id = id_str.parse::<i128>().expect("Expected numerical string");
        println!("id: {:?}", id);

        let mut possible = true;

        let mut next_num = tokens.next().expect("Expected number");
        let mut next_color = tokens.next().expect("Expected color");
        let mut value;
        let mut temp;
        loop {
            value = next_num.parse::<i128>().expect("Expected number");
            for key in maxes.keys() {
                if next_color.starts_with(key) {
                    possible = value <= *maxes.get(key).expect("Key not available");
                }
            }

            if !possible {
                break
            }

            temp = tokens.next();
            if temp.is_none() {
                break;
            }
            next_num = temp.expect("Expected number");
            next_color = tokens.next().expect("Expected color");
        }

        if possible {
            sum += id;
        }
    }

    return sum;
}

pub fn part_2() -> i128 {
    let contents = fs::read_to_string("inputs/day_2.txt")
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');

    let mut sum: i128 = 0;

    for line in lines {
        let mut mins: HashMap<&str, i128> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        println!("{line}");

        let mut tokens = line.split(' ');
        tokens.next();
        let id_str = tokens.next().expect("Expected string").trim_matches(':');
        let id = id_str.parse::<i128>().expect("Expected numerical string");
        println!("id: {:?}", id);

        let mut next_num = tokens.next().expect("Expected number");
        let mut next_color = tokens.next().expect("Expected color");
        let mut value;
        let mut temp;
        loop {
            value = next_num.parse::<i128>().expect("Expected number");
            for key in mins.clone().keys() {
                if next_color.starts_with(key) {
                    mins.insert(key, cmp::max(*mins.get(key).unwrap(), value));
                }
            }

            temp = tokens.next();
            if temp.is_none() {
                break;
            }
            next_num = temp.expect("Expected number");
            next_color = tokens.next().expect("Expected color");
        }

        let prod = mins.get("red").unwrap() * mins.get("blue").unwrap() * mins.get("green").unwrap();
        sum += prod;
        println!("{}, {}, {}", mins.get("red").unwrap(), mins.get("blue").unwrap(), mins.get("green").unwrap())
    }

    return sum;
}