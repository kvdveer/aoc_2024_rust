pub mod puzzle_input;
pub mod puzzle_part1;
pub mod puzzle_part2;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    // Solve the puzzle for each input
    match puzzle_input::PuzzleInput::try_from(puzzle_input) {
        Err(e) => {
            println!("Error: {e:?}");
        }
        Ok(input) => {
            println!("day02 - part 1: {}", puzzle_part1::solve(&input));
            println!("day02 - part 2: {}", puzzle_part2::solve(&input));
        }
    };
}
