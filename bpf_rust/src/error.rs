use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EventError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Invalid event name
    #[error("Invalid Event Name")]
    InvalidEventName,
}

impl From<EventError> for ProgramError {
    fn from(e: EventError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
