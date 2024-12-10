use aoc_grid::Grid;
use nom::{
    self,
    character::complete::{line_ending, multispace0, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    // The raw, unparsed lines of the input. Added here for convenience, in case parsing loses some information.
    pub elevations: Grid<u8>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    let elevation_parser = map(one_of("0123456789."), |c: char| {
        c.to_digit(10).unwrap_or(255) as u8
    });

    // Main parser for the puzzle
    let puzzle_parser = map(
        separated_list1(line_ending, many1(elevation_parser)),
        |cells| PuzzleInput {
            elevations: Grid::from(cells),
        },
    );

    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(multispace0, puzzle_parser, multispace0);

    parser(input)
}

impl TryFrom<&str> for PuzzleInput {
    type Error = Error<String>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match parse_puzzle(s).finish() {
            Ok((_remaining, puzzle_input)) => Ok(puzzle_input),
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
        assert!(input.elevations.width() > 0);
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
