use crate::{vm, optimizer, parser, x86};

pub fn compile_x86(src: &mut impl std::io::Read,
                   out: &mut impl std::io::Write) -> Result<(), String> {
    let mut instr = parse(src)?;
    instr = optimize(instr);
    let x86instr = x86::compile(instr);
    if let Err(e) = x86::generate_asm(x86instr, out) {
        return Err(e.to_string())
    }
    return Ok(());
}

fn optimize(mut instr: Vec<vm::Instruction>) -> Vec<vm::Instruction> {
    loop {
        match optimizer::optimize(&instr) {
            Ok(optimized) => {
                instr = optimized;
            },
            Err(optimizer::MaximumOptimizationReached) => {
                break;
            },
        }
    }
    return instr;
}

fn parse(src: &mut impl std::io::Read) -> Result<Vec<vm::Instruction>, String> {
    match parser::parse(src) {
        Ok(instr) => Ok(instr),
        Err(e) => Err(e.to_string())
    }
}