use crate::vm::Instruction;

#[derive(Debug)]
pub enum ParserError {
    IOError(std::io::Error),
    UnexpectedBracket,
}

pub fn parse(source: &mut impl std::io::Read) -> Result<Vec<Instruction>, ParserError> {
    fn rec_parse(source: &mut impl std::io::Read, in_loop: bool)
                 -> Result<Vec<Instruction>, ParserError> {

        let mut instr = Vec::new();

        loop {
            let mut buffer= [0u8; 1];
            match source.read(&mut buffer) {
                Ok(0) => break,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::Interrupted {
                        continue;
                    }
                    return Err(ParserError::IOError(e));
                }
                _ => (),
            };

            instr.push(match buffer[0] {
                b'+' => Instruction::Add(1, 0),
                b'-' => Instruction::Add(std::u8::MAX, 0),
                b'<' => Instruction::Move(-1),
                b'>' => Instruction::Move(1),
                b'.' => Instruction::Write(0),
                b',' => Instruction::Read(0),
                b'[' => Instruction::Loop(rec_parse(source, true)?),
                b']' => {
                    if in_loop {
                        break;
                    } else {
                        return Err(ParserError::UnexpectedBracket);
                    }
                }
                _ => continue,
            });
        }

        Ok(instr)
    }

    return rec_parse(source, false);
}