use std::collections::HashMap;

fn step(nums: &HashMap<usize, usize>, last: usize, cur_index: usize) -> usize {
    // println!("turn {} {}", cur_index + 1, last);
    match nums.get(&last) {
        Some(pos) => {
            // println!("calc: {} - {} ", cur_index, pos);
            cur_index - pos
        }
        None => {
            // println!("None");
            0
        }
    }
}

pub fn process_part1(input: &str) -> usize {
    let nums: HashMap<usize, usize> = input
        .split(',')
        .map(|v| v.parse::<usize>().expect("valid number"))
        .enumerate()
        .fold(HashMap::new(), |mut map, current| {
            map.insert(current.1, current.0);
            map
        });
    let len = nums.len();
    // let end = *nums.iter().max_by_key(|v| v.1).unwrap().0;
    let result = (nums.len()..)
        .into_iter()
        .scan((nums, 0), |(map, last), i| {
            let next_num = step(&map, *last, i);
            map.insert(*last, i);
            *last = next_num;
            // dbg!(map);
            Some(next_num)
        })
        // .take(10)
        // .inspect(|gen_num| {
        //     dbg!(gen_num);
        // })
        // .collect::<Vec<usize>>();
        .nth(2019 - len - 1)
        .expect("value");
    // dbg!(result)
    result
}

pub fn process_part2(input: &str) -> usize {
    let nums: HashMap<usize, usize> = input
        .split(',')
        .map(|v| v.parse::<usize>().expect("valid number"))
        .enumerate()
        .fold(HashMap::new(), |mut map, current| {
            map.insert(current.1, current.0);
            map
        });
    let len = nums.len();
    let end = *nums.iter().max_by_key(|v| v.1).unwrap().0;
    let result = (nums.len()..)
        .into_iter()
        .scan((nums, 0), |(map, last), i| {
            let next_num = step(&map, *last, i);
            map.insert(*last, i);
            *last = next_num;
            // dbg!(map);
            Some(next_num)
        })
        // .take(20)
        // .enumerate()
        // .inspect(|next| {
        //     dbg!(next.0);
        // })
        // .collect::<Vec<usize>>();
        .nth(30_000_000 - len - 2)
        .expect("value");
    // .1;
    // dbg!(result);
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(process_part1("0,3,6"), 436)
    }
    #[test]
    fn test_input_process_two() {
        assert_eq!(process_part2("0,3,6"), 175594);
        // assert_eq!(process_part2("1,3,2"), 2578);
        // assert_eq!(process_part2("2,1,3"), 3544142);
        // assert_eq!(process_part2("1,2,3"), 261214);
        // assert_eq!(process_part2("2,3,1"), 6895259);
        // assert_eq!(process_part2("3,2,1"), 18);
        // assert_eq!(process_part2("3,1,2"), 362);
    }
}
