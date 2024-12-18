use std::collections::HashSet;

use aoc_grid::Coordinate;

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub falling_bytes: Vec<Coordinate>,
    pub grid_width: usize,
    pub grid_height: usize,
}

impl PuzzleInput {
    pub fn new(falling_bytes: Vec<Coordinate>) -> Self {
        Self {
            grid_width: (falling_bytes.iter().map(|c| c.0).max().unwrap_or(0) + 1) as usize,
            grid_height: (falling_bytes.iter().map(|c| c.1).max().unwrap_or(0) + 1) as usize,
            falling_bytes,
        }
    }

    pub fn validate_assumptions(&self) -> Result<(), String> {
        if HashSet::<&Coordinate>::from_iter(self.falling_bytes.iter()).len()
            != self.falling_bytes.len()
        {
            return Err("Falling bytes must be unique".to_string());
        }

        // Add validation here
        Ok(())
    }
}
