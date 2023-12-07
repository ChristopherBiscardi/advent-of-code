use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(aoc::parse_int_error))]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    CardFromCharError(#[from] CardFromCharError),

    #[error("parse error: splitting strings")]
    #[diagnostic(code(aoc::parse::split))]
    SplitError {
        // The Source that we're gonna be printing snippets out of.
        // This can be a String if you don't have or care about file names.
        #[source_code]
        src: NamedSource,
        // Snippets and highlights can be included in the diagnostic!
        #[label("This bit here")]
        bad_bit: SourceSpan,
    },
}

#[derive(Error, Diagnostic, Debug)]
pub enum CardFromCharError {
    #[error("Not a valid card: `{0}`")]
    #[diagnostic(code(aoc::card::invalid_character))]
    InvalidCharacter(char),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ScoreError {
    #[error("Not a valid card: `{0}`")]
    #[diagnostic(code(aoc::card::invalid_character))]
    InvalidCharacter(char),
}
