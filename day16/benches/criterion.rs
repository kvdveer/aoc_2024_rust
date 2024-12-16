use day16::puzzle::PuzzleInput;
use day16::puzzle_part1::Part1;
use day16::puzzle_part2::Part2;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs = [
        ("example1", include_str!("../example_input1.txt")),
        ("example2", include_str!("../example_input2.txt")),
        ("puzzle", include_str!("../input.txt")),
    ];
    let mut group = c.benchmark_group("day16");
    for (name, puzzle_text) in inputs {
        group.bench_with_input(
            BenchmarkId::new("parse", name),
            puzzle_text,
            |b, puzzle_text| {
                b.iter(|| PuzzleInput::try_from(puzzle_text));
            },
        );

        if let Ok(puzzle) = PuzzleInput::try_from(puzzle_text) {
            group.bench_with_input(BenchmarkId::new("part1", name), &puzzle, |b, puzzle| {
                b.iter(|| puzzle.part1());
            });
            group.bench_with_input(BenchmarkId::new("part2", name), &puzzle, |b, puzzle| {
                b.iter(|| puzzle.part2());
            });
            group.bench_with_input(
                BenchmarkId::new("complete", name),
                puzzle_text,
                |b, puzzle_text| {
                    b.iter(|| {
                        let parsed = PuzzleInput::try_from(puzzle_text).unwrap();
                        parsed.part1();
                        puzzle.part2()
                    });
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
