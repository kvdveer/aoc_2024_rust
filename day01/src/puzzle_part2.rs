use std::collections::HashMap;

use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    // Build a counter hashmap for the second element of each pair
    let mut b = HashMap::new();
    input.pairs.iter().map(|pair| pair.1).for_each(|num| {
        let count = b.entry(num).or_insert(0u64);
        *count += 1;
    });

    // Multiply the first element of each pair by the count in the second list
    let a = input.pairs.iter().map(|pair| pair.0);
    a.map(|a_num| b.get(&a_num).unwrap_or(&0) * a_num)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn test_solve() {
        let input = PuzzleInput::try_from(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve(&input), "31");
    }
}
