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

fn tree_likelyhood(robots: &mut impl Iterator<Item = (i64, i64)>, grid_size: (i64, i64)) -> usize {
    let mut grid = Grid::<bool>::new(grid_size.0 as usize, grid_size.1 as usize);
    // Assumption: when there's a tree, there are several adjacent non-empty cells

    robots
        .filter(|pos| {
            if let Some(c) = grid.get_mut((pos.0 + 1, pos.1)) {
                *c = true;
            }
            if let Some(c) = grid.get_mut((pos.0 - 1, pos.1)) {
                *c = true;
            }
            if let Some(c) = grid.get_mut((pos.0, pos.1 + 1)) {
                *c = true;
            }
            if let Some(c) = grid.get_mut((pos.0, pos.1 - 1)) {
                *c = true;
            }
            grid[pos]
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
        return "UNSOLVABLE".to_string();
    }

    let mut robots = input.robots.clone();
    for i in 0.. {
        let mut robot_locations = robots.iter_mut().map(|robot| {
            robot.simulate_step(1, grid_size);
            robot.position
        });

        let l = tree_likelyhood(&mut robot_locations, grid_size);
        if l > 150 {
            return (i + 1).to_string();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "UNSOLVABLE")]
    #[case::final_input( include_str!("../input.txt"), "7861")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
