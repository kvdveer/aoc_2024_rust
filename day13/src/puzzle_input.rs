use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, multispace0, newline},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClawMachine {
    pub a: Vector,
    pub b: Vector,
    pub target: Vector,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    // The raw, unparsed lines of the input. Added here for convenience, in case parsing loses some information.
    pub claw_machines: Vec<ClawMachine>,
}

fn parse_machine(input: &str) -> IResult<&str, ClawMachine> {
    map(
        tuple((
            tuple((
                preceded(tag("Button A: X+"), nom_i64),
                delimited(tag(", Y+"), nom_i64, newline),
            )),
            tuple((
                preceded(tag("Button B: X+"), nom_i64),
                delimited(tag(", Y+"), nom_i64, newline),
            )),
            tuple((
                preceded(tag("Prize: X="), nom_i64),
                delimited(tag(", Y="), nom_i64, newline),
            )),
        )),
        |(a, b, t)| ClawMachine {
            a: Vector { x: a.0, y: a.1 },
            b: Vector { x: b.0, y: b.1 },
            target: Vector { x: t.0, y: t.1 },
        },
    )(input)
}
fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    // Main parser for the puzzle
    let puzzle_parser = map(
        separated_list1(line_ending, parse_machine),
        |claw_machines| PuzzleInput { claw_machines },
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
        assert!(!input.claw_machines.is_empty());
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
