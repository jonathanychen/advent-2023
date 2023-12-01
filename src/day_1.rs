use std::{fs, collections::{HashSet, HashMap}};

pub fn main() -> i128 {
    let contents = fs::read_to_string("inputs/day_1.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut sum: i128 = 0;

    let digits: HashSet<char> = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let word_digits: HashMap<&str, char> = HashMap::from([("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')]);


    for line in lines {
        println!("{line}");

        let str = line.chars();
        let mut num_str = String::new();

        let mut left_str = String::new();
        for ch in str.clone() {

            left_str.push(ch);
            let mut found: bool = false;
            for key in word_digits.keys() {
                if left_str.ends_with(key) {
                    num_str.push(*word_digits.get(key).unwrap());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }

            if digits.contains(&ch) {
                num_str.push(ch);
                break;
            }
        }

        let mut right_str = String::new();
        for ch in str.clone().rev() {

            right_str.insert(0, ch);
            let mut found: bool = false;
            for key in word_digits.keys() {
                if right_str.starts_with(key) {
                    num_str.push(*word_digits.get(key).unwrap());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }

            if digits.contains(&ch) {
                num_str.push(ch);
                break;
            }
        }

        if num_str.len() == 2 {
            println!("{num_str}");

            sum += num_str.parse::<i128>().unwrap();
            
        }
    }

    return sum;

}