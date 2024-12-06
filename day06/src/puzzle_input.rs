use aoc_grid::Grid;
use nom::{
    character::complete::{line_ending, multispace0, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::delimited,
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MapElement {
    Empty,
    Obstacle,
    Guard,
}

impl From<char> for MapElement {
    fn from(c: char) -> Self {
        match c {
            '.' => MapElement::Empty,
            '#' => MapElement::Obstacle,
            '^' => MapElement::Guard,
            _ => panic!("Invalid grid element: {}", c),
        }
    }
}

impl From<MapElement> for char {
    fn from(e: MapElement) -> Self {
        match e {
            MapElement::Empty => '.',
            MapElement::Obstacle => '#',
            MapElement::Guard => '^',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    // The raw, unparsed lines of the input. Added here for convenience, in case parsing loses some information.
    pub map: Grid<MapElement>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    // Main parser for the puzzle
    let puzzle_parser = map(
        separated_list1(line_ending, many1(map(one_of("#.^"), MapElement::from))),
        |value| PuzzleInput {
            map: Grid::from(value),
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
        assert!(input.map.width() > 0);
        assert!(input.map.height() > 0);
        assert!(
            input
                .map
                .iter()
                .filter(|&e| *e == MapElement::Guard)
                .count()
                == 1
        );
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
