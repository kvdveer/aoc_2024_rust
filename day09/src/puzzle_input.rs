use nom::{
    character::complete::{multispace0, one_of},
    combinator::{map, opt},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::{delimited, tuple},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct File {
    pub index: u64,
    pub size: u8,
    pub space_after: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub files: Vec<File>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    let file_parser = map(
        tuple((one_of("1234567890"), opt(one_of("1234567890")))),
        |(sz, fr)| File {
            index: 0u64, // To be filled by the parser below
            size: sz.to_digit(10).unwrap() as u8,
            space_after: fr.unwrap_or('0').to_digit(10).unwrap() as u8,
        },
    );

    // Main parser for the puzzle
    let puzzle_parser = map(many1(file_parser), |files| PuzzleInput {
        files: files
            .iter()
            .enumerate()
            .map(|(i, f)| File {
                index: i as u64,
                ..f.clone()
            })
            .collect(),
    });

    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(multispace0, puzzle_parser, multispace0);

    parser(input)
}

impl TryFrom<&str> for PuzzleInput {
    type Error = Error<String>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match parse_puzzle(s).finish() {
            Ok(("", puzzle_input)) => Ok(puzzle_input),
            Ok((remaining, _)) => Err(Error {
                input: format!("Remaining input: {}", remaining),
                code: ErrorKind::Fail,
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[rstest]
    #[case::example_input(EXAMPLE_INPUT)]
    #[case::final_input(INPUT)]
    /// Verifies that the test input is valid.
    fn test_puzzle_input_from_example_input(#[case] input: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert!(input.files.len() > 5);
        assert!(!input.files.iter().any(|f| f.size == 0));
    }

    #[rstest]
    #[case::example_input("Not valid input")]
    #[case::final_input("")]
    /// Verifes that invalid input is rejected.
    fn test_puzzle_input_from_str_bad(#[case] input: &str) {
        let input = PuzzleInput::try_from(input);
        assert!(input.is_err());
    }
}
