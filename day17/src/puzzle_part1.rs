use crate::puzzle::{PuzzleInput, VirtualMachine};

pub trait Part1 {
    fn part1(&self) -> String;
}

impl Part1 for PuzzleInput {
    fn part1(&self) -> String {
        let program = self.get_program();
        let vm = VirtualMachine::new(self.registers, &program);
        vm.map(|v| v.to_string()).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case([0,0,9], vec![2,6], [0,1,9], vec![])]
    #[case([10,0,0], vec![5,0,5,1,5,4], [10,0,0], vec![0,1,2])]
    #[case([2024,0,0], vec![0,1,5,4,3,0], [0,0,0], vec![4,2,5,6,7,7,7,7,3,1,0])]
    #[case([0,29,0], vec![1,7], [0,26,0], vec![])]
    #[case([0,2024,43690], vec![4,0], [0,44354,43690], vec![])]
    #[case::main_example([729,0,0], vec![0,1,5,4,3,0], [0,0,0], vec![4,6,3,5,6,3,5,2,1,0])]
    #[case::final_case([23999685,0,0], vec![2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0], [0, 4, 1], vec![5,0,3,5,7,6,1,5,4])]
    fn test_provided_example(
        #[case] registers: [i64; 3],
        #[case] bytecode: Vec<u8>,
        #[case] expected_registers: [i64; 3],
        #[case] expected_output: Vec<u8>,
    ) {
        let input = PuzzleInput {
            registers,
            bytecode,
        };
        let program = input.get_program();
        let mut vm = VirtualMachine::new(input.registers, &program);

        let mut output = Vec::<u8>::new();

        while vm.pc < vm.program.len() {
            let pc = vm.pc;
            if let Some(program_output) = vm.run_step() {
                output.push(program_output);
            }
            println!("#{} {} {:?}", pc, program[pc], vm.registers);
        }

        assert_eq!(vm.registers, expected_registers);
        assert_eq!(output, expected_output);
    }

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "4,6,3,5,6,3,5,2,1,0")]
    #[case::final_input( include_str!("../input.txt"), "5,0,3,5,7,6,1,5,4")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part1(), expected);
    }
}
