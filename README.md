# Learning Rust through Advent of Code

This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) problems, written in [Rust](https://www.rust-lang.org/).

I'm doing these puzzles to learn Rust, and AOC puzzles are my first foray into Rust. I've done some of the 2022 puzzles, so I'm not going into this blank, but still, I'm sure there are many ways to improve the code. I'm open to suggestions!

I'm also not striving to do all the puzzles, as I don't want to neglect my other responsibilities. I'm just going to do them as I have time and interest.


# Operating this environment

* Run `cargo generate --path /template`, and enter the day number (e.g. `day01`) to generate an environment for a new day.
* In one of the day folder:
    * `cargo test` to run the tests.
    * `cargo run` to run the code (which should solve the puzzle).
    * `cargo criterion` or `cargo bench` to run benchmarks.
