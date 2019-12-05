use std::collections::HashMap;

fn main() {
    let mut possible_passwords: Vec<i64> = vec![];
    for num in 245182..=790572 {
        let numStr = num.to_string();

        let num_array: Vec<i64> = numStr.split("")
            .filter(|x| *x != "")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        if is_increasing(&num_array) && has_two_digits(&num_array) {
            possible_passwords.push(num);
        }
    }
    println!("num possible passwords: {}", possible_passwords.len());
}

fn is_increasing(num_array: &Vec<i64>) -> bool {
    let mut last_num = 0;
    let mut result = true;
    for num in num_array {
        if num < &last_num {
            result = false;
        }
        last_num = *num;
    }
    return result;
}

fn has_two_digits(num_array: &Vec<i64>) -> bool {
    let mut counts = HashMap::new();

    for num in num_array {
        let stat = counts.entry(num).or_insert(0);
        *stat += 1;
    }
    for count in counts.values() {
        if *count == 2 {
            return true
        }
    }
    return false
}
