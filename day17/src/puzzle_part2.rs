use crate::puzzle::{OpCode, PuzzleInput, VirtualMachine};

pub trait Part2 {
    fn part2(&self) -> String;
}

fn brute_force(
    program: &Vec<OpCode>,
    target: &Vec<u8>,
    a_register_at_start: i64,
    offset: usize,
) -> Result<i64, ()> {
    // Try each possible permuation for 8 bits
    for permutation in 0..8 {
        // Inject the bits under test into the register
        let value_under_test = a_register_at_start | (permutation << (offset * 3));
        let vm = VirtualMachine::new([value_under_test, 0, 0], program);

        // Run the program
        let mut output = Vec::<_>::with_capacity(target.len());
        output.extend(vm);

        // Pad the output with 0, as repeated zeroes might not be outputted
        output.resize(target.len(), 0);

        if &output == target {
            // We've found the answer
            return Ok(value_under_test);
        }

        if output[offset] == target[offset] {
            // If the current offset matches, then try next offset, too.
            if let Ok(result) = brute_force(program, target, value_under_test, offset - 1) {
                return Ok(result);
            }
        }
    }
    Err(())
}

impl Part2 for PuzzleInput {
    fn part2(&self) -> String {
        // Based on reverse engineering the program, i've determined
        // that the program is processing 3 bits at a time. This means that
        // there are only 8 ways to set the bits. This means that we can brute force each set of 3 bits.

        let program = self.get_program();
        brute_force(&program, &self.bytecode, 0, self.bytecode.len() - 1)
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::final_input( include_str!("../input.txt"), "164516454365621")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part2(), expected);
    }
}
