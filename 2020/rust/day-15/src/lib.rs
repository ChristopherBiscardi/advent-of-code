use itertools::Itertools;

fn step(nums: &[usize]) -> usize {
    if let Some((last, elems)) = nums.split_last() {
        match elems.iter().rev().find(|v| *v == last) {
            Some(pos) => nums.len() - pos - 1,
            None => 0,
        }
    } else {
        panic!("This should never happen")
    }
}

pub fn process_part1(input: &str) -> usize {
    let nums = input
        .split(',')
        .map(|v| v.parse::<usize>().expect("valid number"))
        .collect::<Vec<usize>>();
    let result = (0..)
        .into_iter()
        .scan(vec![], |mut state, i| match nums.get(i) {
            Some(num) => {
                state.push(*num);
                Some(*num)
            }
            None => {
                let next_num = step(state);
                state.push(next_num);
                Some(next_num)
            }
        })
        // .take(20)
        // .inspect(|next| {
        //     dbg!(next);
        // })
        // .collect::<Vec<usize>>();
        .nth(2019)
        .expect("value");
    // dbg!(result);
    result
}

pub fn process_part2(input: &str) -> usize {
    let nums = input
        .split(',')
        .map(|v| v.parse::<usize>().expect("valid number"))
        .collect::<Vec<usize>>();
    let result = (0..)
        .into_iter()
        .scan(vec![], |mut state, i| match nums.get(i) {
            Some(num) => {
                state.push(*num);
                Some(*num)
            }
            None => {
                let next_num = step(state);
                state.push(next_num);
                Some(next_num)
            }
        })
        // .take(20)
        .enumerate()
        .inspect(|next| {
            dbg!(next.0);
        })
        // .collect::<Vec<usize>>();
        .nth(30_000_000)
        .expect("value")
        .1;
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
