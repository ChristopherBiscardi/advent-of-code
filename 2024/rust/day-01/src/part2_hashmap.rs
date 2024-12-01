use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(
            items.next().unwrap().parse::<usize>().unwrap(),
        );
        right
            .entry(
                items
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    let result: usize = left
        .iter()
        .map(|number| {
            number * right.get(number).unwrap_or(&0)
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
