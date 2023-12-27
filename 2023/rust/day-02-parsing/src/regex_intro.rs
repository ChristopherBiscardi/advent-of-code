use crate::game::*;
use regex::Regex;

pub fn parse(input: &str) -> Result<Vec<Game>, String> {
    let re = Regex::new(
        r"(?x)
        Game\s+(\d)+:\s+ # The Game Id
        (.*) # the rounds
        ",
    )
    .unwrap();
    let round_re = Regex::new(
        r"(?x)
    ([^;]+) # any character that isn't a `;`
    (?:;\s)? # uncaptured, optional `; ` ending
    ",
    )
    .unwrap();
    let cube_re = Regex::new(
        r"(?x)
        (?<count>\d+) # cube count
        \s+
        (?<color>\w+) # cube color
        (?:,\s+)? # uncaptured, optional separator",
    )
    .unwrap();

    let results = re.captures_iter(input).map(|c| c.extract())
    .into_iter().map(|(_, [id, rounds])| {
        let rounds = round_re
            .captures_iter(rounds)
            .map(|c| c.extract())
            .into_iter()
            .map(|(_, [round])| {
                cube_re
                    .captures_iter(round)
                    .map(|c| c.extract())
                    .into_iter()
                    .fold(
                        Round::default(),
                        |mut round, (_,[count, color])| {
                            let count =
                                count.parse().unwrap();
                            match color {
                                "red" => {
                                    round.red = count;
                                }
                                "green" => {
                                    round.green = count;
                                }
                                "blue" => {
                                    round.blue = count;
                                }
                                c => unreachable!("should only have red, green, blue colors, got {:?}", c)
                            }
                            round
                        },
                    )
            })
            .collect::<Vec<Round>>();
        Game{ id, rounds }
    }).collect::<Vec<Game>>();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::game_output;

    #[test]
    fn test_parse() {
        let games = parse(&game_output::INPUT).unwrap();
        assert_eq!(game_output::output(), &games);
    }
}
