use puzzle_part1::Part1;
use puzzle_part2::Part2;

pub mod puzzle;
pub mod puzzle_parse;
pub mod puzzle_part1;
pub mod puzzle_part2;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    // Solve the puzzle for each input
    match puzzle::PuzzleInput::try_from(puzzle_input) {
        Err(e) => {
            println!("Error: {e:?}");
        }
        Ok(input) => {
            println!("day15 - part 1: {}", input.part1());
            println!("day15 - part 2: {}", input.part2());
        }
    };
}
