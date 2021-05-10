use crate::error::EventError::InvalidInstruction;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use std::convert::TryInto;

pub enum EventInstruction {
    /// Creates an event by creating an event account and populating it with data.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person creating the event
    /// 1. `[writable]` The event account, it will hold all info about the event.
    /// 2. `[]` The rent sysvar
    /// 3. `[]` The token program
    CreateEvent {
        /// The maximum number of tickets that can be created for this event.
        max_tickets: u64,
    },
    PurchaseTicket {
        purchaser_pubkey: Pubkey,
    },
}

impl EventInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        let result = match tag {
            0 => Self::CreateEvent {
                max_tickets: Self::unpack_max_tickets(rest)?,
            },
            1 => Self::PurchaseTicket {
                purchaser_pubkey: Self::unpack_purchaser(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        };

        Ok(result)
    }

    fn unpack_max_tickets(input: &[u8]) -> Result<u64, ProgramError> {
        let max_tickets = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(max_tickets)
    }

    fn unpack_purchaser(input: &[u8]) -> Result<Pubkey, ProgramError> {
        let purchaser_pubkey = input
            .get(..32)
            .and_then(|slice| slice.try_into().ok())
            .map(Pubkey::new_from_array)
            .ok_or(InvalidInstruction)?;

        Ok(purchaser_pubkey)
    }
}
