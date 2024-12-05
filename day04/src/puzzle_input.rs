use nom::{
    character::complete::{alpha1, line_ending, multispace0},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::delimited,
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    // The raw, unparsed lines of the input. Added here for convenience, in case parsing loses some information.
    pub letters: aoc_grid::Grid<char>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(
        multispace0,
        map(
            separated_list1(line_ending::<&str, _>, alpha1),
            |raw_lines| PuzzleInput {
                letters: aoc_grid::Grid::new_from_iter(
                    raw_lines.len(),
                    raw_lines[0].len(),
                    raw_lines.iter().flat_map(|line| line.chars()),
                ),
            },
        ),
        multispace0,
    );

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
        assert!(input.letters.width() > 1);
    }
}
