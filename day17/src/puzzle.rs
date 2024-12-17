use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub registers: [i64; 3],
    pub bytecode: Vec<u8>,
}

impl PuzzleInput {
    pub fn validate_assumptions(&self) -> Result<(), String> {
        if self.bytecode.len() % 2 != 0 {
            return Err("Invalid number of opcodes".to_string());
        }

        if self.bytecode.chunks(2).any(|pair| pair[1] > 6) {
            return Err("Invalid operand".to_string());
        }

        if self.bytecode.chunks(2).any(|pair| pair[1] > 7) {
            return Err("Invalid opcode".to_string());
        }
        // Add validation here
        Ok(())
    }

    pub fn get_program(&self) -> Vec<OpCode> {
        self.bytecode
            .chunks(2)
            .map(|pair| OpCode::from([pair[0], pair[1]]))
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl From<u8> for Operand {
    fn from(operand: u8) -> Self {
        match operand {
            0..=3 => Operand::Literal(operand),
            4 => Operand::RegisterA,
            5 => Operand::RegisterB,
            6 => Operand::RegisterC,
            _ => unreachable!("Invalid operand"),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(value) => write!(f, "{}", value),
            Operand::RegisterA => write!(f, "A"),
            Operand::RegisterB => write!(f, "B"),
            Operand::RegisterC => write!(f, "C"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
    Adv(Operand),
    Bxl(u8),
    Bst(Operand),
    Jnz(u8),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl From<[u8; 2]> for OpCode {
    fn from(bytecode: [u8; 2]) -> Self {
        match bytecode[0] {
            0 => OpCode::Adv(Operand::from(bytecode[1])),
            1 => OpCode::Bxl(bytecode[1]),
            2 => OpCode::Bst(Operand::from(bytecode[1])),
            3 => OpCode::Jnz(bytecode[1]),
            4 => OpCode::Bxc,
            5 => OpCode::Out(Operand::from(bytecode[1])),
            6 => OpCode::Bdv(Operand::from(bytecode[1])),
            7 => OpCode::Cdv(Operand::from(bytecode[1])),
            _ => unreachable!("Invalid opcode"),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Adv(operand) => write!(f, "ADV {}", operand),
            OpCode::Bxl(value) => write!(f, "BXL {}", value),
            OpCode::Bst(operand) => write!(f, "BST {}", operand),
            OpCode::Jnz(value) => write!(f, "JNZ {}", value),
            OpCode::Bxc => write!(f, "BXC"),
            OpCode::Out(operand) => write!(f, "OUT {}", operand),
            OpCode::Bdv(operand) => write!(f, "BDV {}", operand),
            OpCode::Cdv(operand) => write!(f, "CDV {}", operand),
        }
    }
}

pub struct VirtualMachine<'a> {
    pub registers: [i64; 3],
    pub program: &'a Vec<OpCode>,
    pub pc: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(registers: [i64; 3], program: &'a Vec<OpCode>) -> Self {
        VirtualMachine {
            registers,
            program,
            pc: 0,
        }
    }

    fn interpret_operand(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Literal(value) => *value as i64,
            Operand::RegisterA => self.registers[0],
            Operand::RegisterB => self.registers[1],
            Operand::RegisterC => self.registers[2],
        }
    }

    pub fn run_step(&mut self) -> Option<u8> {
        let opcode = &self.program[self.pc];
        match opcode {
            OpCode::Adv(operand) => {
                self.registers[0] >>= self.interpret_operand(operand);
            }
            OpCode::Bdv(operand) => {
                self.registers[1] = self.registers[0] >> self.interpret_operand(operand);
            }
            OpCode::Cdv(operand) => {
                self.registers[2] = self.registers[0] >> self.interpret_operand(operand);
            }
            OpCode::Bxl(value) => {
                self.registers[1] ^= *value as i64;
            }
            OpCode::Bst(operand) => {
                self.registers[1] = self.interpret_operand(operand) % 8;
            }
            OpCode::Jnz(value) => {
                if self.registers[0] != 0 {
                    self.pc = *value as usize;
                    return None; // prevent the pc from incrementing
                }
            }
            OpCode::Bxc => {
                self.registers[1] ^= self.registers[2];
            }
            OpCode::Out(operand) => {
                self.pc += 1;
                return Some((self.interpret_operand(operand) % 8) as u8);
            }
        };
        self.pc += 1;
        None
    }
}

impl Iterator for VirtualMachine<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        while self.pc < self.program.len() {
            if let Some(output) = self.run_step() {
                return Some(output);
            }
        }
        None
    }
}
