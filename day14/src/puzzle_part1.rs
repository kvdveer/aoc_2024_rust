use crate::puzzle_input::{PuzzleInput, Robot};

trait RobotSimulator {
    fn simulate(&self, seconds: i64, grid_size: (i64, i64)) -> (i64, i64);
}

impl RobotSimulator for Robot {
    fn simulate(&self, seconds: i64, grid_size: (i64, i64)) -> (i64, i64) {
        (
            (self.position.0 + self.velocity.0 * seconds).rem_euclid(grid_size.0),
            (self.position.1 + self.velocity.1 * seconds).rem_euclid(grid_size.1),
        )
    }
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

    // Grid should be odd-sized
    assert!(grid_size.0 % 2 == 1);
    assert!(grid_size.1 % 2 == 1);

    let center = ((grid_size.0) / 2, (grid_size.1) / 2);

    input
        .robots
        .iter()
        // Simulate the robots
        .map(|robot| robot.simulate(100, grid_size))
        .filter_map(|(x, y)| {
            // Compute tje quadrant for each robot
            if x < center.0 && y < center.1 {
                //                println!("Q0 {},{}", x, y);
                Some(0)
            } else if x > center.0 && y < center.1 {
                //                println!("Q1 {},{}", x, y);
                Some(1)
            } else if x < center.0 && y > center.1 {
                //                println!("Q2 {},{}", x, y);
                Some(2)
            } else if x > center.0 && y > center.1 {
                //                println!("Q3 {},{}", x, y);
                Some(3)
            } else {
                None
            }
        })
        .fold([0, 0, 0, 0], |mut acc, q| {
            acc[q] += 1;
            acc
        })
        .iter()
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc_grid::Grid;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "12")]
    #[case::final_input( include_str!("../input.txt"), "229868730")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }

    #[rstest]
    #[case::example_robot(Robot{position: (2, 4), velocity: (2, -3)}, 1, (4, 1))]
    #[case::example_robot(Robot{position: (2, 4), velocity: (2, -3)}, 2, (6, 5))]
    fn test_robot_simulate(
        #[case] robot: Robot,
        #[case] seconds: i64,
        #[case] expected: (i64, i64),
    ) {
        assert_eq!(robot.simulate(seconds, (11, 7)), expected);
    }

    #[test]
    fn test_example() {
        let robot_grid = [
            "......2..1.",
            "...........",
            "1..........",
            ".11........",
            ".....1.....",
            "...12......",
            ".1....1....",
        ];

        let reference_location_grid = Grid::<i32>::new_from_iter(
            robot_grid[0].len(),
            robot_grid.len(),
            robot_grid
                .iter()
                .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap_or(0) as i32)),
        );

        // dbg!(reference_location_grid.map(|c| c.to_string().chars().next().unwrap()));

        let puzzle = PuzzleInput::try_from(include_str!("../example_input.txt")).unwrap();

        let locations = puzzle
            .robots
            .iter()
            .map(|robot| robot.simulate(100, (11, 7)));

        let mut computed_location_grid = Grid::<i32>::new(11, 7);
        for (x, y) in locations {
            computed_location_grid[(x, y)] += 1;
        }
        dbg!(computed_location_grid.map(|c| c.to_string().chars().next().unwrap()));
        assert!(reference_location_grid == computed_location_grid);
    }
}
