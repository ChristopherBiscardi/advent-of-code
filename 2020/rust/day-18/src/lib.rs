mod libpart1;
mod libpart2;
use libpart1::Calc;

// 2 * 3 + (4 * 5)
// ("2" "*" "3" "+" ("4" "*" "5"))
// type Span<'a> = LocatedSpan<&'a str>;

fn process(bootstrap: usize, it: std::slice::Iter<Calc>) -> usize {
    it.fold((bootstrap, None), |(acc, acc_op), current| match current {
        Calc::Operator(op) => (acc, Some(op)),
        Calc::Number(num) => match acc_op {
            Some(&"+") => (acc + num, None),
            Some(&"*") => (acc * num, None),
            _ => panic!("have to process number, but no operator to work with"),
        },
        Calc::SubCalcs(calcs) => {
            let last = fold_calc(calcs);
            match acc_op {
                Some(&"+") => (acc + last, None),
                Some(&"*") => (acc * last, None),
                _ => panic!("have to process calcs, but no operator to work with"),
            }
        }
    })
    .0
}
fn fold_calc(input: &Vec<Calc>) -> usize {
    let mut it = input.iter();
    match it.next() {
        Some(Calc::Number(first_num)) => process(*first_num, it),
        Some(Calc::SubCalcs(calcs)) => process(fold_calc(&calcs), it),
        v => {
            panic!("asfklj")
        }
    }
}

pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| fold_calc(&libpart1::parse(line).unwrap()))
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    libpart2::solve()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_26() {
        assert_eq!(process_part1("2 * 3 + (4 * 5)"), 26)
    }
    #[test]
    fn test_input_process_437() {
        assert_eq!(process_part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437)
    }
    #[test]
    fn test_input_process_12240() {
        assert_eq!(
            process_part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        )
    }
    #[test]
    fn test_input_process_13632() {
        assert_eq!(
            process_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        )
    }
    // part 2

    #[test]
    fn test_input_process2_51() {
        assert_eq!(process_part2("1 + (2 * 3) + (4 * (5 + 6))"), 51)
    }
    #[test]
    fn test_input_process2_26() {
        assert_eq!(process_part2("2 * 3 + (4 * 5)"), 46)
    }
    #[test]
    fn test_input_process2_437() {
        assert_eq!(process_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445)
    }
    #[test]
    fn test_input_process2_12240() {
        assert_eq!(
            process_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        )
    }
    #[test]
    fn test_input_process2_13632() {
        assert_eq!(
            process_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        )
    }
}
