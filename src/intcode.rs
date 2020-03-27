use crate::digits::digits;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub enum IntcodeError {
    InvalidOpcodeOperation,
    NegativeOpcode,
    InvalidAddressingMode,
    NegativeAddress,
    InvalidStoreAddressingMode,
    DidNotRunToCompletion,
}

#[derive(Debug, PartialEq)]
pub enum StopStatus {
    Running,
    Halted,
    BlockedOnInput,
}

enum AddressingMode {
    AbsoluteAddress,
    Immediate,
    BasePointerRelative,
}

struct Operand {
    mode: AddressingMode,
    value: isize,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    AdjustBasePointer,
    Halt,
}

struct Opcode {
    operation: Operation,
    operands: Vec<Operand>,
}

pub type IntcodeResult<T> = Result<T, IntcodeError>;
pub type Tape = Vec<isize>;
pub type Stream = VecDeque<isize>;
pub type StreamRef = Rc<RefCell<Stream>>;

pub struct IntcodeMachine {
    tape: Tape,
    pub input: StreamRef,
    pub output: StreamRef,
    pc: isize,
    bp: isize,
}

fn parse_addressing_mode(digit: usize) -> IntcodeResult<AddressingMode> {
    match digit {
        0 => Ok(AddressingMode::AbsoluteAddress),
        1 => Ok(AddressingMode::Immediate),
        2 => Ok(AddressingMode::BasePointerRelative),
        _ => Err(IntcodeError::InvalidAddressingMode),
    }
}

impl IntcodeMachine {
    pub fn new_io(tape: Tape, input: StreamRef, output: StreamRef) -> Self {
        IntcodeMachine {
            tape,
            input,
            output,
            pc: 0,
            bp: 0,
        }
    }

    pub fn new(tape: Tape) -> Self {
        Self::new_io(tape, new_stream_ref(), new_stream_ref())
    }

    fn verify_addr(&mut self, addr: isize) -> IntcodeResult<usize> {
        if addr < 0 {
            return Err(IntcodeError::NegativeAddress);
        }

        let addr = addr as usize;
        if addr >= self.tape.len() {
            self.tape.resize(addr + 1, 0);
        }
        Ok(addr)
    }

    pub fn read_addr(&mut self, addr: isize) -> IntcodeResult<isize> {
        let addr = self.verify_addr(addr)?;
        Ok(self.tape[addr])
    }

    fn write_addr(&mut self, addr: isize, value: isize) -> IntcodeResult<()> {
        let addr = self.verify_addr(addr)?;
        self.tape[addr] = value;
        Ok(())
    }

    fn read_pc(&mut self) -> IntcodeResult<isize> {
        let value = self.read_addr(self.pc)?;
        self.pc += 1;
        Ok(value)
    }

    fn read_opcode(&mut self) -> IntcodeResult<Opcode> {
        let opcode = self.read_pc()?;
        if opcode < 0 {
            return Err(IntcodeError::NegativeOpcode);
        }

        let mut digits = digits(opcode as usize, 10);
        digits.extend(vec![0; 5 - digits.len()]);

        let (operation, operand_count) = match 10 * digits[1] + digits[0] {
            1 => (Operation::Add, 3),
            2 => (Operation::Multiply, 3),
            3 => (Operation::Input, 1),
            4 => (Operation::Output, 1),
            5 => (Operation::JumpTrue, 2),
            6 => (Operation::JumpFalse, 2),
            7 => (Operation::LessThan, 3),
            8 => (Operation::Equals, 3),
            9 => (Operation::AdjustBasePointer, 1),
            99 => (Operation::Halt, 0),
            _ => return Err(IntcodeError::InvalidOpcodeOperation),
        };

        let mut operands = Vec::<Operand>::new();
        for i in 0..operand_count {
            let mode = parse_addressing_mode(digits[2 + i])?;
            let value = self.read_pc()?;
            operands.push(Operand { mode, value });
        }

        Ok(Opcode {
            operation,
            operands,
        })
    }

    fn load(&mut self, op: &Operand) -> IntcodeResult<isize> {
        match op.mode {
            AddressingMode::AbsoluteAddress => Ok(self.read_addr(op.value)?),
            AddressingMode::Immediate => Ok(op.value),
            AddressingMode::BasePointerRelative => Ok(self.read_addr(self.bp + op.value)?),
        }
    }

    fn store(&mut self, op: &Operand, value: isize) -> IntcodeResult<()> {
        match op.mode {
            AddressingMode::AbsoluteAddress => Ok(self.write_addr(op.value, value)?),
            AddressingMode::BasePointerRelative => Ok(self.write_addr(self.bp + op.value, value)?),
            AddressingMode::Immediate => Err(IntcodeError::InvalidStoreAddressingMode),
        }
    }

    fn jump_conditional(&mut self, condition: bool, target: isize) -> IntcodeResult<()> {
        if condition {
            self.verify_addr(target)?;
            self.pc = target;
        }
        Ok(())
    }

    fn tick(&mut self) -> IntcodeResult<StopStatus> {
        let start_pc = self.pc;
        let opcode = self.read_opcode()?;

        match opcode.operation {
            Operation::Add => {
                let value = self.load(&opcode.operands[0])? + self.load(&opcode.operands[1])?;
                self.store(&opcode.operands[2], value)?;
            }
            Operation::Multiply => {
                let value = self.load(&opcode.operands[0])? * self.load(&opcode.operands[1])?;
                self.store(&opcode.operands[2], value)?;
            }
            Operation::Input => {
                let input = self.input.borrow_mut().pop_front();
                match input {
                    Some(value) => self.store(&opcode.operands[0], value)?,
                    None => {
                        self.pc = start_pc;
                        return Ok(StopStatus::BlockedOnInput);
                    }
                };
            }
            Operation::Output => {
                let value = self.load(&opcode.operands[0])?;
                self.output.borrow_mut().push_back(value);
            }
            Operation::JumpTrue => {
                let condition = self.load(&opcode.operands[0])?;
                let target = self.load(&opcode.operands[1])?;
                self.jump_conditional(condition != 0, target)?;
            }
            Operation::JumpFalse => {
                let condition = self.load(&opcode.operands[0])?;
                let target = self.load(&opcode.operands[1])?;
                self.jump_conditional(condition == 0, target)?;
            }
            Operation::LessThan => {
                let value = self.load(&opcode.operands[0])? < self.load(&opcode.operands[1])?;
                self.store(&opcode.operands[2], value as isize)?;
            }
            Operation::Equals => {
                let value = self.load(&opcode.operands[0])? == self.load(&opcode.operands[1])?;
                self.store(&opcode.operands[2], value as isize)?;
            }
            Operation::AdjustBasePointer => {
                let newbp = self.bp + self.load(&opcode.operands[0])?;
                self.verify_addr(newbp)?;
                self.bp = newbp;
            }
            Operation::Halt => {
                self.pc = start_pc;
                return Ok(StopStatus::Halted);
            }
        };

        Ok(StopStatus::Running)
    }

    pub fn run(&mut self) -> IntcodeResult<StopStatus> {
        loop {
            match self.tick() {
                Ok(StopStatus::Running) => continue,
                Ok(status) => return Ok(status),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn run_to_completion(&mut self) -> IntcodeResult<()> {
        match self.run()? {
            StopStatus::Halted => Ok(()),
            _ => Err(IntcodeError::DidNotRunToCompletion),
        }
    }
}

pub fn new_stream_ref() -> StreamRef {
    Rc::new(RefCell::new(Stream::new()))
}

pub fn new_stream_ref_from(value: isize) -> StreamRef {
    let s = new_stream_ref();
    s.borrow_mut().push_back(value);
    s
}

pub fn new_stream_ref_from_iter(iter: impl IntoIterator<Item = isize>) -> StreamRef {
    let s = new_stream_ref();
    s.borrow_mut().extend(iter);
    s
}

pub fn parse_intcode_program(input: &str) -> Tape {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Tape>()
}
