use day04::{puzzle_input, puzzle_part1, puzzle_part2};
use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench]
fn parsing() {
    let puzzle_input = include_str!("../input.txt");
    puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();
}

#[divan::bench]
fn parsing_example() {
    let puzzle_input = include_str!("../example_input.txt");
    puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();
}

#[divan::bench]
fn part1(bencher: Bencher) {
    let puzzle_input = include_str!("../input.txt");
    let parsed = puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();
    bencher.bench_local(|| puzzle_part1::solve(divan::black_box(&parsed)));
}

#[divan::bench]
fn part2(bencher: Bencher) {
    let puzzle_input = include_str!("../input.txt");
    let parsed = puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();

    bencher.bench_local(|| puzzle_part2::solve(divan::black_box(&parsed)));
}

#[divan::bench]
fn part1example(bencher: Bencher) {
    let puzzle_input = include_str!("../example_input.txt");
    let parsed = puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();
    bencher.bench_local(|| puzzle_part1::solve(divan::black_box(&parsed)));
}

#[divan::bench]
fn part2example(bencher: Bencher) {
    let puzzle_input = include_str!("../example_input.txt");
    let parsed = puzzle_input::PuzzleInput::try_from(divan::black_box(puzzle_input)).unwrap();

    bencher.bench_local(|| puzzle_part2::solve(divan::black_box(&parsed)));
}
