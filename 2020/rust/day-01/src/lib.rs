use itertools::Itertools;

pub fn process(input: &str) -> Option<u64> {
    let list = input
        .lines()
        .map(|string_num| string_num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    list.iter()
        .cartesian_product(list.iter())
        .cartesian_product(list.iter())
        .find(|((a, b), c)| **a + **b + **c == 2020)
        .map(|((a, b), c)| a * b * c)
}
