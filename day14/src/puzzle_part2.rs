use crate::puzzle_input::{PuzzleInput, Robot};
use aoc_grid::Grid;

trait RobotSimulator {
    fn simulate_step(&mut self, seconds: i64, grid_size: (i64, i64));
}

impl RobotSimulator for Robot {
    fn simulate_step(&mut self, seconds: i64, grid_size: (i64, i64)) {
        self.position.0 = (self.position.0 + self.velocity.0 * seconds).rem_euclid(grid_size.0);
        self.position.1 = (self.position.1 + self.velocity.1 * seconds).rem_euclid(grid_size.1);
    }
}

fn tree_likelyhood(grid: &Grid<i32>) -> usize {
    // Assumption: when there's a tree, there are several adjacent non-empty cells
    grid.iter_pairs()
        .filter_map(|(c, v)| if *v > 0 { Some(c) } else { None })
        .filter(|c| {
            *grid.get((c.0 + 1, c.1)).unwrap_or(&0) != 0
                || *grid.get((c.0 - 1, c.1)).unwrap_or(&0) != 0
                || *grid.get((c.0, c.1 + 1)).unwrap_or(&0) != 0
                || *grid.get((c.0, c.1 - 1)).unwrap_or(&0) != 0
        })
        .count()
}

pub fn solve(input: &PuzzleInput) -> String {
    let grid_size = (
        input
            .robots
            .iter()
            .map(|robot| robot.position.0)
            .max()
            .unwrap()
            + 1,
        input
            .robots
            .iter()
            .map(|robot| robot.position.1)
            .max()
            .unwrap()
            + 1,
    );

    if grid_size.0 < 50 {
        // I don't think the example has a solution
        return "UNSOLVED".to_string();
    }

    let mut robots = input.robots.clone();
    for i in 0.. {
        let mut computed_location_grid =
            Grid::<i32>::new(grid_size.0 as usize, grid_size.1 as usize);
        robots
            .iter_mut()
            .map(|robot| {
                robot.simulate_step(1, grid_size);
                robot.position
            })
            .for_each(|pos| {
                computed_location_grid[pos] += 1;
            });

        let l = tree_likelyhood(&computed_location_grid);
        if l > 300 {
            return (i + 1).to_string();
        }
    }
    "UNSOLVED".to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "12")]
    #[case::final_input( include_str!("../input.txt"), ">500,<242932500")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
