use crate::error::EventError::InvalidInstruction;
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};

use std::convert::TryInto;
use std::str;

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
        name: String,
    },
    PurchaseTicket {
        purchaser_pubkey: Pubkey,
    },
}

impl EventInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpack instruction.");
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        msg!("Rest: {:?}", rest);

        let result = match tag {
            0 => {
                msg!("CreateEvent instruction");
                Self::CreateEvent {
                    name: Self::unpack_name(rest)?,
                    max_tickets: Self::unpack_max_tickets(rest)?,
                }
            }
            1 => {
                msg!("PurchaseTicket instruction");
                Self::PurchaseTicket {
                    purchaser_pubkey: Self::unpack_purchaser(rest)?,
                }
            }
            _ => return Err(InvalidInstruction.into()),
        };

        Ok(result)
    }

    fn unpack_name(input: &[u8]) -> Result<String, ProgramError> {
        msg!("Unpack name {:?}", input);
        let name = input
            .get(..32)
            .and_then(|slice| slice.try_into().ok())
            .map(str::from_utf8)
            .ok_or(InvalidInstruction)?
            .map_err(|_| InvalidInstruction)?;
        msg!("Unpack name finished: {}", name);
        Ok(name.into())
    }

    fn unpack_max_tickets(input: &[u8]) -> Result<u64, ProgramError> {
        msg!("Unpack max tickets: {:?}", input);
        let max_tickets = input
            .get(32..)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        msg!("Input now: {:?}", input);
        Ok(max_tickets)
    }

    fn unpack_purchaser(input: &[u8]) -> Result<Pubkey, ProgramError> {
        msg!("Unpack purchaser");
        let purchaser_pubkey = input
            .get(..32)
            .and_then(|slice| slice.try_into().ok())
            .map(Pubkey::new_from_array)
            .ok_or(InvalidInstruction)?;

        Ok(purchaser_pubkey)
    }
}
