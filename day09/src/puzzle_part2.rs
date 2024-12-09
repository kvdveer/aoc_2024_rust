use std::{collections::BTreeMap, ops::Range};

use crate::puzzle_input::PuzzleInput;

struct PlacedFile {
    index: u64,
    start: u64,
    len: u8,
}

struct UsedMap {
    used: BTreeMap<u64, Range<u64>>,
}

impl UsedMap {
    fn new(_size: u64) -> UsedMap {
        UsedMap {
            used: BTreeMap::new(),
        }
    }

    fn mark(&mut self, area: Range<u64>, value: bool) {
        if value {
            self.used.insert(area.start, area);
        } else {
            let old = self.used.remove(&area.start).unwrap();
            assert!(old.start == area.start);
            assert!(old.end == area.end);
        }
    }

    fn find_free(&mut self, len: u8, max_pos: u64) -> Option<u64> {
        let mut cursor = 0;
        for (start, range) in self.used.iter() {
            if *start > max_pos {
                return None;
            }

            if range.start - cursor >= len as u64 {
                return Some(cursor);
            }

            cursor = range.end;
        }
        return None;
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    let mut cursor = 0u64;
    let mut files = input
        .files
        .iter()
        .map(|f| {
            let result = PlacedFile {
                index: f.index,
                start: cursor,
                len: f.size,
            };
            cursor += (f.size + f.space_after) as u64;
            result
        })
        .collect::<Vec<_>>();

    let last_file = files.last().unwrap();
    let disk_size = last_file.start + last_file.len as u64;

    let mut free_map = UsedMap::new(disk_size);
    files.iter().for_each(|file| {
        free_map.mark(file.start..(file.start + file.len as u64), true);
    });

    files.iter_mut().rev().for_each(|file| {
        if let Some(spot) = free_map.find_free(file.len, file.start) {
            // Mark the previous location of the file as free
            free_map.mark(file.start..(file.start + file.len as u64), false);

            // Move the file to the free space.
            file.start = spot;

            // Mark the free space as used.
            free_map.mark(file.start..(file.start + file.len as u64), true);
        }
    });

    // Calculate checksum
    files
        .iter()
        .flat_map(|file| (file.start..file.start + file.len as u64).map(|i| i * file.index))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "2858")]
    #[case::final_input( include_str!("../input.txt"), "6478232739671")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
