use itertools::Itertools;

pub fn process(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|string_num| string_num.parse::<u64>().unwrap())
        .combinations(3)
        .find(|perm| perm.iter().sum::<u64>() == 2020)
        .map(|v| v.iter().product())
}
