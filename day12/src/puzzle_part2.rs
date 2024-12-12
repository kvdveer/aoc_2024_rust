use aoc_grid::{Coordinate, Direction, Grid};

use crate::grid_follow::FollowArea;
use crate::puzzle_input::PuzzleInput;

fn count_corners(center: &Coordinate, map: &Grid<char>) -> usize {
    let base_value = map[center];
    (0..4)
        .map(|rot| {
            let z = [
                Direction::Left.rotate_clockwise_4(rot),
                Direction::UpLeft.rotate_clockwise_4(rot),
                Direction::Up.rotate_clockwise_4(rot),
            ];

            let c = z.map(|f| map.get(*center + &f) == Some(&base_value));
            // Collect values for each of the corners. This yields an array of booleans
            // representing the corners in the following order:
            // 10
            // 2#  where # is the corner under investigation

            match c {
                // Outside corner (also with adjacent field)
                //  .
                // .#   where # is the corner under investigation
                [false, _, false] => 1,

                // Inside corner
                // .A   wher A matches the corner under investigation
                // A#   where # is the corner under investigation
                [true, false, true] => 1,

                _ => 0,
            }
        })
        .sum()
}

pub fn solve(input: &PuzzleInput) -> String {
    let mut visited = Grid::<bool>::new(input.map.width(), input.map.height());

    let mut result = 0usize;
    for coordinate in input.map.iter_coordinates() {
        if visited[&coordinate] {
            continue;
        }
        visited[&coordinate] = true;

        let field_value = &input.map[&coordinate];

        let area = input
            .map
            .follow_area(&coordinate, |a| a == field_value)
            .inspect(|f| visited[f] = true)
            .collect::<Vec<_>>();

        let area_circumference = area
            .iter()
            .map(|c| count_corners(c, &input.map))
            .sum::<usize>();

        result += area_circumference * area.len();
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // #[case::example_input(include_str!("../example_input.txt"), "1930")]
    #[case::example_input(include_str!("../example_input_1.txt"), "80")]
    #[case::example_input(include_str!("../example_input_2.txt"), "436")]
    #[case::example_input(include_str!("../example_reddit.txt"), "946")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "1494342")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
