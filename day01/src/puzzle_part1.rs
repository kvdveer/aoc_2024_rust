use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    // Split the pairs into two separate vectors
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input.pairs.iter().cloned().unzip();

    // Sort each vector
    a.sort();
    b.sort();

    let differences: i32 = a
        // Re-pair
        .iter()
        .zip(b.iter())
        // Calculate the difference between each pair
        .map(|(a, b)| a.abs_diff(*b) as i32)
        // Sum the differences
        .sum::<i32>();

    differences.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn test_solve() {
        let input = PuzzleInput::try_from(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve(&input), "11");
    }
}
