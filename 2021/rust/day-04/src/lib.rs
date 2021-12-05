use ndarray::{
    arr2, aview_mut2, Array2, ArrayView, ArrayView2,
    ArrayViewMut2,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::opt,
    multi::{fill, separated_list1},
    IResult,
};

#[derive(Debug)]
pub struct Board<'a> {
    board: Array2<BoardSquare<'a>>,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mark {
    Called,
    Uncalled,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardSquare<'a> {
    num: &'a str,
    called: Mark,
}

#[derive(Debug)]
pub enum BoardState {
    Finished(u32),
    Playing,
}
impl<'a> Board<'a> {
    pub fn score(&self) -> u32 {
        self.board
            .iter()
            .filter_map(|BoardSquare { num, called }| {
                match called {
                    Mark::Called => None,
                    Mark::Uncalled => {
                        Some(num.parse::<u32>().unwrap())
                    }
                }
            })
            .sum()
    }
    // todo: return
    pub fn mark(&mut self, callout: &str) -> BoardState {
        // mark all cells that match callout
        for mut row in self.board.rows_mut() {
            // dbg!(&row);
            for i in 0..row.len() {
                if row.get(i).unwrap().num == callout {
                    row.get_mut(i).unwrap().called =
                        Mark::Called;
                }
            }
        }
        let row_win =
            self.board.rows().into_iter().find(|row| {
                row.indexed_iter().all(|(_, item)| {
                    item.called == Mark::Called
                })
            });
        let column_win =
            self.board.columns().into_iter().find(|row| {
                row.indexed_iter().all(|(_, item)| {
                    item.called == Mark::Called
                })
            });

        match (row_win, column_win) {
            (None, None) => BoardState::Playing,
            (None, Some(row))
            | (Some(row), None)
            | (Some(row), Some(_)) => BoardState::Finished(
                self.score()
                    * callout.parse::<u32>().unwrap(),
            ),
        }
    }
}
fn square(input: &str) -> IResult<&str, BoardSquare> {
    let (input, value) = digit1(input)?;
    let (input, _) = opt(space1)(input)?;
    Ok((
        input,
        BoardSquare {
            num: value,
            called: Mark::Uncalled,
        },
    ))
}
pub fn row(input: &str) -> IResult<&str, [BoardSquare; 5]> {
    let (input, _) = opt(newline)(input)?;
    let (input, _) = opt(space1)(input)?;
    let mut buf = [BoardSquare {
        num: "",
        called: Mark::Uncalled,
    }; 5];
    let (input, ()) = fill(square, &mut buf)(input)?;
    Ok((input, buf))
}
pub fn board(input: &str) -> IResult<&str, Board> {
    let mut rows = [[BoardSquare {
        num: "",
        called: Mark::Uncalled,
    }; 5]; 5];
    let (input, ()) = fill(row, &mut rows)(input)?;

    let arr = arr2(&rows);
    Ok((input, Board { board: arr }))
}

pub fn separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}

pub fn puzzle_input(
    input: &str,
) -> IResult<&str, (Vec<&str>, Vec<Board>)> {
    let (input, callouts) =
        separated_list1(tag(","), digit1)(input)?;
    let (input, _) = separator(input)?;
    let (input, boards) =
        separated_list1(separator, board)(input)?;
    Ok((input, (callouts, boards)))
}
