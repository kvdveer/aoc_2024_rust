use aoc_grid::Direction;

use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    let search_directions = [
        Direction::UpLeft,
        Direction::DownLeft,
        Direction::DownRight,
        Direction::UpRight,
    ];

    input
        .letters
        // Check each grid cell
        .iter_coordinates()
        // Find the center "A"
        .filter(|coord| input.letters[coord] == 'A')
        // Find the surrounding characters
        .filter(|coord| {
            let n = search_directions
                .iter()
                .map(|dir| input.letters.get(*coord + dir))
                // Early out for bad characters
                .take_while(|&c| c == Some(&'S') || c == Some(&'M'))
                .map(|c| *c.expect("checked above"))
                .collect::<String>();

            n.len() == 4 && n == "MMSS" || n == "SSMM" || n == "SMMS" || n == "MSSM"
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "9")]
    #[case::final_input( include_str!("../input.txt"), "1824")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
