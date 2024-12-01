use {{project-name}}::{puzzle_input, puzzle_part1, puzzle_part2};

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs = [
        ("test", include_str!("../example_input.txt")),
        ("puzzle", include_str!("../input.txt")),
    ];
    let mut group = c.benchmark_group("{{project-name}}");
    for (name, input) in inputs {
        group.bench_with_input(BenchmarkId::new("parse", name), input, |b, input| {
            b.iter(|| puzzle_input::PuzzleInput::try_from(input));
        });

        if let Ok(parsed) = puzzle_input::PuzzleInput::try_from(input) {
            group.bench_with_input(BenchmarkId::new("part1", name), &parsed, |b, parsed| {
                b.iter(|| puzzle_part1::solve(parsed));
            });
            group.bench_with_input(BenchmarkId::new("part2", name), &parsed, |b, parsed| {
                b.iter(|| puzzle_part2::solve(parsed));
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
