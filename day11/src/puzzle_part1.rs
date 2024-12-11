use crate::puzzle_input::PuzzleInput;

struct ExpansionIterator<'t> {
    source: &'t mut dyn Iterator<Item = &'t u128>,
    to_yield: Option<u128>,
}

impl<'t> ExpansionIterator<'t> {
    fn new(source: &'t mut dyn Iterator<Item = &'t u128>) -> Self {
        Self {
            source,
            to_yield: None,
        }
    }
}

impl Iterator for ExpansionIterator<'_> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(yielded) = self.to_yield.take() {
            return Some(yielded);
        }

        let next = self.source.next()?;
        if *next == 0 {
            return Some(1);
        }

        let digits = next.ilog10() + 1;

        if digits % 2 == 0 {
            // Even number of digits, split the number in half
            let splitting_power = 10u128.pow(digits / 2);
            self.to_yield = Some(next % splitting_power);

            Some(next / splitting_power)
        } else {
            Some(next * 2024)
        }
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    let mut finger = input.numbers.clone();

    for _ in 0..25 {
        finger = ExpansionIterator::new(&mut finger.iter()).collect();
    }

    finger.len().to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "55312")]
    #[case::final_input( include_str!("../input.txt"), "202019")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
