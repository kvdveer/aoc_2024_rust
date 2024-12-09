use crate::puzzle_input::PuzzleInput;

struct PlacedFile {
    index: u64,
    start: u64,
    len: u8,
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

    let mut free_list = input
        .files
        .iter()
        .flat_map(|f| {
            (0..f.size)
                .map(|_| false)
                .chain((0..f.space_after).map(|_| true))
        })
        .collect::<Vec<_>>();

    files.iter_mut().rev().for_each(|file| {
        cursor = 0;
        let mut free_count = 0;
        let mut free_start = 0;
        let mut free_spot: Option<u64> = None;

        for (i, is_free) in free_list.iter().enumerate() {
            if (i as u64) > file.start {
                // Only move files to the left.
                break;
            }
            if !is_free {
                free_count = 0;
                free_start = i + 1;
            } else {
                free_count += 1;
                if free_count == file.len as u64 {
                    free_spot = Some(free_start as u64);
                    break;
                }
            }
        }

        if let Some(spot) = free_spot {
            // Mark the previous location of the file as free
            free_list[(file.start as usize)..(file.start + file.len as u64) as usize]
                .iter_mut()
                .for_each(|f| *f = true);

            // Move the file to the free space.
            file.start = spot;

            // Mark the free space as used.
            free_list[(file.start as usize)..(file.start + file.len as u64) as usize]
                .iter_mut()
                .for_each(|f| *f = false);
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
