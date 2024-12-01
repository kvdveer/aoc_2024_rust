use crate::puzzle_input::PuzzleInput;

pub fn solve(_input: &PuzzleInput) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    #[ignore]
    fn test_solve() {
        let input = PuzzleInput::try_from(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve(&input), "");
    }
}
