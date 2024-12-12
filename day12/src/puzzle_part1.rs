use aoc_grid::{Direction, Grid};

use crate::grid_follow::FollowArea;
use crate::puzzle_input::PuzzleInput;

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
            .map(|c| {
                Direction::CARDINAL_4
                    .iter()
                    .filter(|&dir| input.map.get(*c + dir) != Some(field_value))
                    .count()
            })
            .sum::<usize>();

        result += area_circumference * area.len();

        // println!(
        //     "field: {:?}, Area: {:?}, Circumference: {}",
        //     field_value,
        //     area.len(),
        //     area_circumference
        // );

        // let area_visual = Grid::<bool>::new_from_iter(
        //     input.map.width(),
        //     input.map.height(),
        //     input.map.iter_coordinates().map(|c| area.contains(&c)),
        // );

        // println!(
        //     "{}",
        //     area_visual.map(|&v| if v { *field_value } else { ' ' })
        // );
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "1930")]
    #[case::example_input(include_str!("../example_input_1.txt"), "140")]
    #[case::example_input(include_str!("../example_input_2.txt"), "772")]
    #[case::example_input(include_str!("../example_reddit.txt"), "2566")]
    #[case::final_input( include_str!("../input.txt"), "1494342")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
