#[derive (Debug, Clone, PartialEq)]
pub enum Instruction {
    Add(u8, isize),
    Move(isize),
    Loop(Vec<Instruction>),
    Read(isize),
    Write(isize),
    Assign(u8, isize),
}

#[derive (Debug)]
pub enum RuntimeError {
    OutOfMemoryBounds
}

const MEMORY_SIZE: usize = 30000;

pub struct VM {
    memory: [u8; MEMORY_SIZE],
    cursor: usize
}

impl VM {
    pub fn run(code: &Vec<Instruction>) -> Result<u8, RuntimeError> {

        let mut vm = VM::new();

        fn rec_run(vm: &mut VM, code: &Vec<Instruction>) -> Result<(), RuntimeError> {
            for instr in code {
                match instr {
                    Instruction::Add(v, offset) => {
                        let loc = vm.compute_offset(*offset)?;
                        let (sum, _) = v.overflowing_add(vm.memory[loc]);
                        vm.memory[loc] = sum;
                    },
                    Instruction::Move(offset) => {
                        let loc = vm.compute_offset(*offset)?;
                        vm.cursor = loc;
                    },
                    Instruction::Loop(inner_instr) => {
                        while vm.memory[vm.cursor] > 0 {
                            rec_run(vm, inner_instr)?;
                        }
                    },
                    Instruction::Read(_) => {
                        unimplemented!(); //TODO proper read & write operations implementation
                    },
                    Instruction::Write(offset) => {
                        print!("{}", vm.memory[vm.compute_offset(*offset)?] as char);
                    },
                    Instruction::Assign(v, offset) => {
                        let loc = vm.compute_offset(*offset)?;
                        vm.memory[loc] = *v;
                    }
                }
            }

            Ok(())
        }

        rec_run(&mut vm, code)?;

        Ok(vm.memory[vm.cursor])
    }

    fn compute_offset(&self, offset: isize) -> Result<usize, RuntimeError> {
        let loc: isize = self.cursor as isize + offset;
        if loc < 0 || loc >= MEMORY_SIZE as isize {
            Err(RuntimeError::OutOfMemoryBounds)
        } else {
            Ok(loc as usize)
        }
    }

    fn new() -> Self {
        VM {
            memory: [0u8; MEMORY_SIZE],
            cursor: 0
        }
    }
}