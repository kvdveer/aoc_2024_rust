use nom::{
    self,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, space1},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Equation {
    pub outcome: u64,
    pub operands: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub equations: Vec<Equation>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    let equasion_parser = map(
        separated_pair(
            nom::character::complete::u64,
            tag(": "),
            separated_list1(space1, nom::character::complete::u64),
        ),
        |(outcome, operands)| Equation { outcome, operands },
    );

    // Main parser for the puzzle
    let puzzle_parser = map(separated_list1(line_ending, equasion_parser), |equations| {
        PuzzleInput { equations }
    });
    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(multispace0, puzzle_parser, multispace0);

    parser(input)
}

impl TryFrom<&'_ str> for PuzzleInput {
    type Error = Error<String>;

    fn try_from(s: &'_ str) -> Result<Self, Self::Error> {
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
        assert!(!input.equations.is_empty());
        assert!(input
            .equations
            .iter()
            .all(|equation| !equation.operands.is_empty()));
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
