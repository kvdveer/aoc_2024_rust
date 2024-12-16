use aoc_grid::{Coordinate, Grid};
use nom::Finish;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MapState {
    Wall,
    Empty,
    Start,
    Finish,
}

impl From<char> for MapState {
    fn from(c: char) -> Self {
        match c {
            '#' => MapState::Wall,
            '.' => MapState::Empty,
            'S' => MapState::Start,
            'E' => MapState::Finish,
            _ => panic!("Invalid map state: {}", c),
        }
    }
}

impl From<MapState> for char {
    fn from(state: MapState) -> Self {
        match state {
            MapState::Wall => '#',
            MapState::Empty => '.',
            MapState::Start => 'S',
            MapState::Finish => 'E',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub map: Grid<MapState>,
    pub start: Coordinate,
    pub finish: Coordinate,
}

impl PuzzleInput {
    pub fn new(mut map: Grid<MapState>) -> Self {
        let start = map
            .iter_pairs()
            .find_map(|(coord, &state)| {
                if state == MapState::Start {
                    Some(coord)
                } else {
                    None
                }
            })
            .expect("No start found");
        let finish = map
            .iter_pairs()
            .find_map(|(coord, &state)| {
                if state == MapState::Finish {
                    Some(coord)
                } else {
                    None
                }
            })
            .expect("No finish found");

        map[start] = MapState::Empty;
        map[finish] = MapState::Empty;

        Self { map, start, finish }
    }

    pub fn validate_assumptions(&self) -> Result<(), String> {
        if self.map.iter().any(|&state| state == MapState::Start) {
            return Err("More than one start found".to_string());
        }
        if self.map.iter().any(|&state| state == MapState::Finish) {
            return Err("More than one finish found".to_string());
        }

        Ok(())
    }
}
