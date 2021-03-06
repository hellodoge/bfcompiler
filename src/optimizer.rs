use crate::vm::Instruction;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

#[derive (Debug)]
struct State {
    /// Hasher should be default to prevent "fake"
    /// optimizations caused by instructions rearrangement
    mem: HashMap<isize, (u8, MemoryCellState), BuildHasherDefault<DefaultHasher>>,
    cursor_offset: isize,
}

#[derive (Copy, Clone, Debug)]
enum MemoryCellState {
    Assigned,
    Relative,
}

fn get_state(code: &Vec<Instruction>) -> State {
    let mut state = State {
        mem: HashMap::default(),
        cursor_offset: 0,
    };

    for instr in code {
        match instr {
            Instruction::Add(x, offset) => {
                let (prev_value, cell_state) =
                    state.mem.get(&(state.cursor_offset + *offset))
                        .or(Some(&(0u8, MemoryCellState::Relative))).unwrap().clone();
                let (sum, _) = x.overflowing_add(prev_value);
                state.mem.insert(state.cursor_offset + *offset, (sum, cell_state));
            }
            Instruction::Move(offset) => {
                state.cursor_offset += *offset;
            }
            Instruction::Assign(x, offset) => {
                state.mem.insert(state.cursor_offset + *offset, (*x, MemoryCellState::Assigned));
            }
            _ => unreachable!()
        }
    }

    return state
}

impl State {
    fn get_instructions(&self) -> Vec<Instruction> {
        let mut optimized = Vec::new();
        for (offset, (val, state)) in self.mem.iter() {
            optimized.push(match state {
                MemoryCellState::Relative if *val != 0 => {
                    Instruction::Add(*val, *offset)
                }
                MemoryCellState::Assigned => {
                    Instruction::Assign(*val, *offset)
                }
                _ => continue
            })
        }
        if self.cursor_offset != 0 {
            optimized.push(Instruction::Move(self.cursor_offset));
        }

        return optimized;
    }
}

fn optimize_loop(instructions: Vec<Instruction>) -> Instruction {
    if instructions.len() == 1 {
        return match instructions.into_iter().nth(0).unwrap() {
            Instruction::Add(1, 0) | Instruction::Add(255, 0) | Instruction::Assign(0, 0) => {
                Instruction::Assign(0, 0)
            }
            Instruction::Loop(x) => {
                optimize_loop(x)
            }
            other => Instruction::Loop(vec![other]),
        }
    }
    return Instruction::Loop(instructions);
}

fn optimize_instructions(code: &Vec<Instruction>) -> Vec<Instruction> {
    let mut optimized = Vec::new();
    let mut set = Vec::new();
    let mut prev_state: Option<State> = None;
    for (i, instr) in code.iter().enumerate() {
        let mut not_part_of_set = false;
        match instr {
            Instruction::Add(_, _) | Instruction::Move(_) | Instruction::Assign(_, _) => {
                set.push(instr.clone());
                if i + 1 != code.len() { continue }
            },
            _ => { not_part_of_set = true }
        }
        if !set.is_empty() {
            let state = get_state(&set);
            optimized.extend(state.get_instructions());
            set.clear();
            prev_state = Some(state);
        }
        if not_part_of_set {
            match instr {
                Instruction::Loop(inner) => {
                    if let Some(state) = &prev_state {
                        match state.mem.get(&state.cursor_offset) {
                            Some((value, state)) => {
                                match *state {
                                    MemoryCellState::Assigned if *value == 0 => continue,
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    if prev_state.is_none() {
                        continue
                    }
                    optimized.push(
                        optimize_loop(optimize_instructions(inner))
                    )
                }
                _ => optimized.push(instr.clone())
            }
        }
    }

    return optimized;
}

pub struct MaximumOptimizationReached;

pub fn optimize(code: &Vec<Instruction>) -> Result<Vec<Instruction>, MaximumOptimizationReached> {
    let optimized_instr = optimize_instructions(code);
    if &optimized_instr != code {
        Ok(optimized_instr)
    } else {
        Err(MaximumOptimizationReached)
    }
}