use solana_program::{instruction::Instruction, program_error::ProgramError};

pub type InstructionResult = Result<Instruction, ProgramError>;
