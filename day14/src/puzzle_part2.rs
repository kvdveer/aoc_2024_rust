use std::vec;

use crate::puzzle_input::{PuzzleInput, Robot};

trait RobotSimulator {
    fn simulate_step(&mut self, seconds: i64, grid_size: (i64, i64));
}

impl RobotSimulator for Robot {
    fn simulate_step(&mut self, seconds: i64, grid_size: (i64, i64)) {
        self.position.0 = (self.position.0 + self.velocity.0 * seconds).rem_euclid(grid_size.0);
        self.position.1 = (self.position.1 + self.velocity.1 * seconds).rem_euclid(grid_size.1);
    }
}

fn contains_ascii_art(
    robots: &mut impl Iterator<Item = (i64, i64)>,
    grid_size: (i64, i64),
) -> bool {
    let mut robot_count = 0;

    let tally = robots.fold(
        (vec![0; grid_size.0 as usize], vec![0; grid_size.1 as usize]),
        |mut tally, (x, y)| {
            robot_count += 1;
            tally.0[x as usize] += 1;
            tally.1[y as usize] += 1;
            tally
        },
    );

    tally.0.iter().any(|&v| v > 5 * robot_count / grid_size.0)
        && tally.1.iter().any(|&v| v > 5 * robot_count / grid_size.1)
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
    for i in 0..10000 {
        let mut robot_locations = robots.iter_mut().map(|robot| {
            robot.simulate_step(1, grid_size);
            robot.position
        });

        if contains_ascii_art(&mut robot_locations, grid_size) {
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
