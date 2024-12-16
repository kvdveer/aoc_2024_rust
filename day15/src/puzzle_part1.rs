use crate::puzzle::PuzzleInput;

pub trait Part1 {
    fn part1(&self) -> String;
}

impl Part1 for PuzzleInput {
    fn part1(&self) -> String {
        let mut s = self.clone();

        s.simulate_all();
        s.coordinate_sum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input1.txt"), "10092")]
    #[case::example_input(include_str!("../example_input2.txt"), "2028")]
    #[case::final_input( include_str!("../input.txt"), "1577255")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part1(), expected);
    }
}
