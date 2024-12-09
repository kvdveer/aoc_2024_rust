use std::collections::VecDeque;

use crate::puzzle_input::{File, PuzzleInput};

struct Defragger {
    files: VecDeque<File>,
    head_file: File,
    tail_file: File,
}

impl Defragger {
    fn new(files: &[File]) -> Self {
        let mut files = files.iter().cloned().collect::<VecDeque<_>>();
        let head_file = files.pop_front().unwrap();
        let tail_file = files.pop_back().unwrap();
        Self {
            files,
            head_file,
            tail_file,
        }
    }
}

impl Iterator for Defragger {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head_file.size == 0 && self.head_file.space_after == 0 {
            // We've consumed the current file, find the next one.
            if let Some(f) = self.files.pop_front() {
                self.head_file = f;
            } else {
                // No more files available, finish consuming the tail file.
                self.head_file = self.tail_file.clone();
                self.head_file.space_after = 0; // There's no space after we're done
                self.tail_file.size = 0; // Tail file is transfered to head file
            }
        }

        if self.head_file.size > 0 {
            // Currently in a file that's not moved.
            self.head_file.size -= 1;
            return Some(self.head_file.index);
        }

        if self.head_file.space_after > 0 {
            // Currently in the space after an unmoved, file, fill it with the tail file.
            self.head_file.space_after -= 1;
            if self.tail_file.size == 0 {
                match self.files.pop_back() {
                    Some(f) => self.tail_file = f,
                    None => {
                        // All done!
                        return None;
                    }
                }
            }
            self.tail_file.size -= 1;
            return Some(self.tail_file.index);
        }

        None
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    Defragger::new(&input.files)
        .enumerate()
        .map(|(i, f)| i as u64 * f)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "1928")]
    #[case::example_input("22222", "19")]
    #[case::final_input( include_str!("../input.txt"), "6446899523367")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
