use crate::{vm, util};

#[derive(Debug, Clone)]
pub enum Operand {
    MemoryOffset(i32),
    Register(&'static str),
    ConstI32(i32),
    ConstU32(u32),
    ConstU8(u8)
}

const POSITION_REGISTER: Operand = Operand::Register("esi");
const EAX: Operand = Operand::Register("eax");
const EBX: Operand = Operand::Register("ebx");
const ECX: Operand = Operand::Register("ecx");
const EDX: Operand = Operand::Register("edx");

const STDIN: Operand = Operand::ConstI32(0);
const STDOUT: Operand = Operand::ConstI32(1);
const SYS_READ: Operand = Operand::ConstI32(3);
const SYS_WRITE: Operand = Operand::ConstI32(4);

/// intel syntax order
#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Jif(&'static str, String),
    Mov(Operand, Operand),
    Label(String),
    Cmp(Operand, Operand),
    Int(u8)
}

const SYSTEM_CALL: Instruction = Instruction::Int(0x80);

pub fn compile(instr: Vec<vm::Instruction>) -> Vec<Instruction> {

    fn compile_instr(instr: vm::Instruction) -> Vec<Instruction> {

        fn add_or_dec_instruction(dst: Operand, offset: isize) -> Vec<Instruction> {
            if offset < 0 {
                return vec![Instruction::Sub(
                    dst,
                    Operand::ConstU32(-offset as u32),
                )]
            } else if offset > 0 {
                return vec![Instruction::Add(
                    dst,
                    Operand::ConstU32(offset as u32),
                )]
            };
            return Vec::new()
        }

        match instr {
            vm::Instruction::Add(value, offset) => {
                vec![Instruction::Add(
                    Operand::ConstU8(value),
                    Operand::MemoryOffset(offset as i32)
                )]
            }
            vm::Instruction::Assign(value, offset) => {
                vec![Instruction::Mov(
                    Operand::MemoryOffset(offset as i32),
                    Operand::ConstU8(value)
                )]
            }
            vm::Instruction::Move(offset) => {
                add_or_dec_instruction(POSITION_REGISTER, offset)
            }
            vm::Instruction::Loop(instr) => {
                let unique_id = util::get_unique();
                let opening = ".L_OPEN_".to_owned() + &unique_id;
                let closing = ".L_CLOSE_".to_owned() + &unique_id;
                [
                    vec![
                        Instruction::Cmp(POSITION_REGISTER, Operand::ConstU8(0)),
                        Instruction::Jif("jz", closing.clone()),
                        Instruction::Label(opening.clone())
                    ],
                    compile(instr),
                    vec![
                        Instruction::Cmp(POSITION_REGISTER, Operand::ConstU8(0)),
                        Instruction::Jif("jnz", opening),
                        Instruction::Label(closing)
                    ]
                ].concat()
            }
            vm::Instruction::Write(offset) => {
                [
                    vec![
                        Instruction::Mov(EAX, SYS_WRITE),
                        Instruction::Mov(EBX, STDOUT),
                        Instruction::Mov(ECX, Operand::ConstI32(1)),
                        Instruction::Mov(EDX, POSITION_REGISTER),
                    ],
                    add_or_dec_instruction(EDX, offset),
                    vec![
                        SYSTEM_CALL,
                    ]
                ].concat()
            }
            vm::Instruction::Read(offset) => {
                [
                    vec![
                        Instruction::Mov(EAX, SYS_READ),
                        Instruction::Mov(EBX, STDIN),
                        Instruction::Mov(ECX, Operand::ConstI32(1)),
                        Instruction::Mov(EDX, POSITION_REGISTER),
                    ],
                    add_or_dec_instruction(EDX, offset),
                    vec![
                        SYSTEM_CALL,
                    ]
                ].concat()
            }
        }
    }

    let mut compiled = Vec::new();
    for instruction in instr {
        compiled.extend(compile_instr(instruction));
    }
    return compiled;
}

const MEM_LABEL: &'static str = "mem";

pub fn generate_asm(program: Vec<Instruction>, out: &mut impl std::io::Write) -> std::io::Result<()> {
    write!(out, "global _start\n")?;
    write!(out, "section .bss\n")?;
    write!(out, "\t{} resb {}\n", MEM_LABEL, vm::MEMORY_SIZE)?;
    write!(out, "section .text\n")?;
    write!(out, "_start:\n")?;
    for instr in program {
        write!(out, "{}\n", instr)?;
    }
    Ok(())
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add(sum, term) =>
                write!(f, "\tadd\t{},\t{}", sum, term),
            Instruction::Mov(to, from) =>
                write!(f, "\tmov\t{},\t{}", to, from),
            Instruction::Sub(dif, term) =>
                write!(f, "\tsub\t{},\t{}", dif, term),
            Instruction::Cmp(l, r) =>
                write!(f, "\tcmp\t{},\t{}", l, r),
            Instruction::Jif(instr, label) =>
                write!(f, "\t{}\t{}", *instr, *label),
            Instruction::Label(label) =>
                write!(f, "{}", label),
            Instruction::Int(int) =>
                write!(f, "int {:x}", int)
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::ConstI32(op) => write!(f, "dword {}", *op),
            Operand::ConstU32(op) => write!(f, "dword {}", *op),
            Operand::Register(reg) => write!(f, "{}", *reg),
            Operand::ConstU8(op) => write!(f, "byte {}", *op),
            Operand::MemoryOffset(offset) => {
                if *offset < 0 {
                    write!(f, "[{} - {}]", POSITION_REGISTER, -*offset)
                } else {
                    write!(f, "[{} + {}]", POSITION_REGISTER, *offset)
                }
            }
        }
    }
}