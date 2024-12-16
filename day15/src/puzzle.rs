use std::{collections::VecDeque, fmt::Display};

use aoc_grid::{grid_index::GridIndex, Coordinate, Direction, Grid};

/// A part of the map.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GridElement {
    /// Empty space, which something could be shoved into
    Empty,
    /// A wall, blocks movement.
    Wall,
    // Movable object
    Box,
    /// The left part of a wide box (part 2)
    BoxLeft,
    /// The right part of a wide box (part 2)
    BoxRight,
    /// Initial robot location, removed after initial parsing.
    Robot,
}

impl From<char> for GridElement {
    fn from(c: char) -> Self {
        match c {
            '#' => GridElement::Wall,
            'O' => GridElement::Box,
            '[' => GridElement::BoxLeft,
            ']' => GridElement::BoxRight,
            '.' => GridElement::Empty,
            '@' => GridElement::Robot,
            _ => panic!("Invalid character"),
        }
    }
}

impl From<GridElement> for char {
    fn from(e: GridElement) -> Self {
        match e {
            GridElement::Wall => '#',
            GridElement::Box => 'O',
            GridElement::Empty => '.',
            GridElement::Robot => '@',
            GridElement::BoxLeft => '[',
            GridElement::BoxRight => ']',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub grid: Grid<GridElement>,
    pub robot: Coordinate,
    pub instructions: VecDeque<Direction>,
}

impl PuzzleInput {
    pub fn new(grid: Grid<GridElement>, instructions: Vec<Direction>) -> Self {
        // Extract the robot position from the grid
        let robot = grid
            .iter_pairs()
            .find(|(_, c)| **c == GridElement::Robot)
            .unwrap()
            .0;
        let mut grid = grid.clone();

        // Make the grid position empty
        grid[robot] = GridElement::Empty;

        PuzzleInput {
            grid,
            robot,
            instructions: instructions.into(),
        }
    }

    pub fn validate_assumptions(&self) -> Result<(), String> {
        if self.grid.iter().any(|c| *c == GridElement::Robot) {
            return Err("More than one Robot found".to_string());
        }

        if self.grid[self.robot] != GridElement::Empty {
            return Err("Robot is not on an empty space".to_string());
        }

        // There should be walls on all the edges of the grid
        for x in 0..self.grid.width() {
            if self.grid[(x, 0)] != GridElement::Wall {
                return Err("Wall missing on the top edge".to_string());
            }
            if self.grid[(x, self.grid.height() - 1)] != GridElement::Wall {
                return Err("Wall missing on the bottom edge".to_string());
            }
        }
        for y in 0..self.grid.height() {
            if self.grid[(0, y)] != GridElement::Wall {
                return Err("Wall missing on the left edge".to_string());
            }
            if self.grid[(self.grid.width() - 1, y)] != GridElement::Wall {
                return Err("Wall missing on the right edge".to_string());
            }
        }
        Ok(())
    }

    pub fn simulate_one_step(&mut self) {
        // Find out the next direction to move
        let direction = self.instructions.pop_front().unwrap();
        let mut coordinates_to_move = vec![self.robot + &direction];

        let mut finger = 0usize;

        while finger < coordinates_to_move.len() {
            let next_position = coordinates_to_move[finger];
            if self.grid[next_position] == GridElement::Wall {
                // Wall cannot be moved
                return;
            }

            let new_positions = self.expand_move(next_position, direction);

            new_positions.iter().for_each(|c| {
                if !coordinates_to_move.contains(c) {
                    coordinates_to_move.push(*c);
                }
            });
            finger += 1;
        }

        coordinates_to_move.iter().rev().for_each(|&c| {
            if self.grid[c] != GridElement::Empty {
                assert!(self.grid[c + &direction] == GridElement::Empty);
                self.grid[c + &direction] = self.grid[c];
                self.grid[c] = GridElement::Empty
            }
        });
        self.robot += &direction;
    }

    pub fn expand_move(&self, position: Coordinate, direction: Direction) -> Vec<Coordinate> {
        match self.grid[position] {
            GridElement::Wall => vec![], // Wall cannot be moved
            GridElement::Box => {
                vec![position + &direction]
            }
            GridElement::Empty => vec![],
            GridElement::Robot => unreachable!(),
            GridElement::BoxLeft => match direction {
                Direction::Up | Direction::Down => {
                    vec![position + &Direction::Right, position + &direction]
                }
                _ => vec![position + &direction],
            },
            GridElement::BoxRight => match direction {
                Direction::Up | Direction::Down => {
                    vec![position + &Direction::Left, position + &direction]
                }
                _ => vec![position + &direction],
            },
        }
    }

    pub fn simulate_all(&mut self) {
        while !self.instructions.is_empty() {
            self.simulate_one_step();
        }
    }

    pub fn coordinate_sum(&self) -> isize {
        self.grid
            .iter_pairs()
            .map(|(c, e)| match e {
                GridElement::Box | GridElement::BoxLeft => c.x() + 100 * c.y(),
                _ => 0,
            })
            .sum()
    }

    pub fn expand(&self) -> PuzzleInput {
        let expanded_grid = self.grid.iter().flat_map(|&c| match c {
            GridElement::Wall => [GridElement::Wall, GridElement::Wall],
            GridElement::Box => [GridElement::BoxLeft, GridElement::BoxRight],
            GridElement::BoxLeft => unreachable!(),
            GridElement::BoxRight => unreachable!(),
            GridElement::Empty => [GridElement::Empty, GridElement::Empty],
            GridElement::Robot => unreachable!(),
        });
        PuzzleInput {
            grid: Grid::new_from_iter(self.grid.width() * 2, self.grid.height(), expanded_grid),
            robot: Coordinate(self.robot.x() * 2, self.robot.y()),
            instructions: self.instructions.clone(),
        }
    }
}

impl Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = self.grid.map(|c| char::from(*c));
        formatted[self.robot] = '@';
        write!(f, "{}", formatted)?;
        writeln!(f)?;
        write!(
            f,
            "{}",
            self.instructions
                .iter()
                .map(|d| match d {
                    Direction::Up => '^',
                    Direction::Right => '>',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    _ => unreachable!(),
                })
                .collect::<String>()
        )?;

        Ok(())
    }
}

mod test {
    use crate::puzzle::PuzzleInput;

    #[test]
    fn test_simulate_one_step_free() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "####\n", //
            "#@.#\n", //
            "####\n", //
            "\n",     //
            "><"      //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "####\n", //
                "#.@#\n", //
                "####\n", //
                "\n",     //
                "<"       //
            )
        );
    }
    #[test]
    fn test_simulate_one_step_box_wall() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "####\n", //
            "#@O#\n", //
            "####\n", //
            "\n",     //
            "><"      //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "####\n", //
                "#@O#\n", //
                "####\n", //
                "\n",     //
                "<"       //
            )
        );
    }
    #[test]
    fn test_simulate_one_step_box_down() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "###\n", //
            "#@#\n", //
            "#O#\n", //
            "#.#\n", //
            "###\n", //
            "\n",    //
            "v<"     //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "###\n", //
                "#.#\n", //
                "#@#\n", //
                "#O#\n", //
                "###\n", //
                "\n",    //
                "<"      //
            )
        );
    }

    #[test]
    fn test_simulate_one_step_box() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "#####\n", //
            "#@O.#\n", //
            "#####\n", //
            "\n",      //
            "><"       //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "#####\n", //
                "#.@O#\n", //
                "#####\n", //
                "\n",      //
                "<"        //
            )
        );
    }
    #[test]
    fn test_simulate_one_step_wide_box_right() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "######\n", //
            "#@[].#\n", //
            "######\n", //
            "\n",       //
            "><"        //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "######\n", //
                "#.@[]#\n", //
                "######\n", //
                "\n",       //
                "<"         //
            )
        );
    }

    #[test]
    fn test_simulate_one_step_wide_box_down() {
        let mut puzzle = PuzzleInput::try_from(concat!(
            "#####\n", //
            "#@..#\n", //
            "#[].#\n", //
            "#...#\n", //
            "#####\n", //
            "\n",      //
            "v<"       //
        ))
        .unwrap();

        puzzle.simulate_one_step();

        assert_eq!(
            puzzle.to_string(),
            concat!(
                "#####\n", //
                "#...#\n", //
                "#@..#\n", //
                "#[].#\n", //
                "#####\n", //
                "\n",      //
                "<"        //
            )
        );
    }
}
