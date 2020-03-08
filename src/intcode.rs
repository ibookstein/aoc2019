use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[repr(usize)]
#[derive(TryFromPrimitive)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

pub type Tape = Vec<usize>;

pub struct IntcodeMachine {
    pub tape: Tape,
    pub pc: usize,
}

impl IntcodeMachine {
    pub fn new(tape: Tape) -> IntcodeMachine {
        IntcodeMachine { tape: tape, pc: 0 }
    }

    fn tick(&mut self) -> bool {
        let pc = self.pc as usize;
        if pc + 3 >= self.tape.len() {
            return false;
        }

        let mut opcode_bytes = [0usize; 4];
        opcode_bytes.copy_from_slice(&self.tape[pc..=pc + 3]);
        self.pc += opcode_bytes.len();

        let [opcode, src1, src2, dst] = opcode_bytes;

        let opcode = match Opcode::try_from(opcode) {
            Ok(op) => op,
            Err(_) => return false,
        };

        match opcode {
            Opcode::Add => self.tape[dst] = self.tape[src1] + self.tape[src2],
            Opcode::Mul => self.tape[dst] = self.tape[src1] * self.tape[src2],
            _ => return false,
        }

        true
    }

    pub fn run(&mut self) {
        while self.tick() {}
    }
}
