use indicatif::ProgressIterator;
use std::cmp::{max, min};

use crate::puzzle_input::{ClawMachine, PuzzleInput};

trait Solver {
    fn solve(&self) -> i64;
}

impl Solver for ClawMachine {
    fn solve(&self) -> i64 {
        // Find a starting point on the X axis that maximizes B (the cheaper option)
        // Do not go over 100 (per puzzle instructions)
        let mut b = min(self.target.x / self.b.x + 1, 100);
        // find how much remains on the x axis
        let remaining = max(self.target.x - b * self.b.x, 0);
        let mut a = min(remaining / self.a.x + 1, 100);

        while b >= 0 {
            let x = a * self.a.x + b * self.b.x;
            let y = a * self.a.y + b * self.b.y;
            if x == self.target.x && y == self.target.y {
                return 3 * a + b;
            }

            if x > self.target.x {
                b -= 1;
            } else {
                a += 1;
            }
        }

        0
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .claw_machines
        .iter()
        .progress()
        .map(|m| m.solve())
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "480")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
