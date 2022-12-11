use std::{collections::VecDeque, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, preceded},
    *,
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}
impl Display for Value {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Old => "itself".to_string(),
                Value::Num(num) => num.to_string(),
            }
        )
    }
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}
#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recipient: u64,
    false_recipient: u64,
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(
        &mut self,
        relief_lowers_worry_level: bool,
        magic_trick: u64,
    ) -> u64 {
        self.touch_count += 1;
        let item = self.items.pop_front().unwrap();
        // println!("  Monkey inspects an item with a worry level of {item}.");
        let worry_level = match &self.operation {
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a * num_b;
                // println!("    Worry level is multiplied by {b} to {result}.");
                result % magic_trick
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a + num_b;
                // println!("    Worry level increases by {b} to {result}.");
                result % magic_trick
            }
        };
        let result = if relief_lowers_worry_level {
            worry_level / 3
        } else {
            worry_level
        };
        // println!("    Monkey gets bored with item. Worry level is divided by 3 to {result}.");
        result
    }
    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {
            // println!("    Current worry level is divisible by {}.", self.test.divisible);
            self.test.true_recipient
        } else {
            // println!("    Current worry level is not divisible by {}.", self.test.divisible);
            self.test.false_recipient
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64
            .map(|num| Value::Num(num)),
    ))(input)
}
fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(
        multispace1,
        alt((tag("*"), tag("+"))),
        multispace1,
    )(input)?;
    let (input, value_2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown operator"),
    };
    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(
        tag("Test: divisible by "),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recipient) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recipient) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;
    Ok((
        input,
        Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    ))
}
fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(
        tag("Monkey "),
        nom::character::complete::u64,
        tag(":"),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(
            tag(", "),
            nom::character::complete::u64,
        ),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            test,
            touch_count: 0,
        },
    ))
}

// Monkey 0:
//   Monkey inspects an item with a worry level of 79.
//     Worry level is multiplied by 19 to 1501.
//     Monkey gets bored with item. Worry level is divided by 3 to 500.
//     Current worry level is not divisible by 23.
//     Item with worry level 500 is thrown to monkey 3.
pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) =
        separated_list1(tag("\n\n"), monkey)(input)
            .unwrap();
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();
    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            // println!("Monkey {monkey_index}:");
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey =
                    monkeys.get_mut(monkey_index).unwrap();
                let item =
                    monkey.inspect(true, magic_trick);
                let monkey_to_send_to = monkey.test(item);
                // println!("    Item with worry level {item} is thrown to monkey {monkey_to_send_to}.");
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut monkeys) =
        separated_list1(tag("\n\n"), monkey)(input)
            .unwrap();
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();

    for _ in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            // println!("Monkey {monkey_index}:");
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey =
                    monkeys.get_mut(monkey_index).unwrap();
                let item =
                    monkey.inspect(false, magic_trick);
                let monkey_to_send_to = monkey.test(item);
                // println!("    Item with worry level {item} is thrown to monkey {monkey_to_send_to}.");
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "10605");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "2713310158");
    }
}
