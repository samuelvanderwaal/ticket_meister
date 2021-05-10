use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::EventError;
use crate::instruction::EventInstruction;
use crate::state::Event;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EventInstruction::unpack(instruction_data)?;

        match instruction {
            EventInstruction::CreateEvent { max_tickets } => {
                msg!("Instruction: CreateEvent");
                msg!("Max tickets: {}", max_tickets);
                Ok(())
            }
            EventInstruction::PurchaseTicket { purchaser_pubkey } => {
                msg!("Instruction: PurchaseTicket");
                msg!("Purchaser: {}", purchaser_pubkey);
                Ok(())
            }
        }
    }

    fn process_create_event(
        accounts: &[AccountInfo],
        max_tickets: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let event_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(event_account.lamports(), event_account.data_len()) {
            return Err(EventError::NotRentExempt.into());
        }

        // Deserialize current event account data into a Rust struct.
        let mut event_info = Event::unpack_unchecked(&event_account.data.borrow())?;

        // Ensure account isn't already initialized.
        if event_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Set new values.
        event_info.is_initialized = true;
        event_info.initializer_pubkey = *initializer.key;
        event_info.max_tickets = max_tickets;

        // Serialize back into account data to update the account.
        Event::pack(event_info, &mut event_account.data.borrow_mut())?;

        Ok(())
    }
}
