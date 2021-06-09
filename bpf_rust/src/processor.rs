use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction::initialize_mint;

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
        msg!("PROCESSING");
        let instruction = EventInstruction::unpack(instruction_data)?;

        match instruction {
            EventInstruction::CreateEvent { max_tickets, name } => {
                msg!("Instruction: CreateEvent");
                msg!("max_tickets: {}", max_tickets);
                Self::process_create_event(accounts, max_tickets, name, program_id)
            }
            // EventInstruction::PurchaseTicket { purchaser_pubkey } => {
            EventInstruction::PurchaseTicket => {
                msg!("Instruction: PurchaseTicket");
                // msg!("Purchaser: {}", purchaser_pubkey);
                Self::process_purchase_ticket(accounts, program_id)
            }
        }
    }

    fn process_create_event(
        accounts: &[AccountInfo],
        max_tickets: u64,
        name: String,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let event_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        msg!("Before rent.");
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        msg!("Before rent exempt.");
        if !rent.is_exempt(event_account.lamports(), event_account.data_len()) {
            return Err(EventError::NotRentExempt.into());
        }

        let token_program = next_account_info(account_info_iter)?;

        // let mint_token_ix = initialize_mint(
        //     token_program.key,
        //     mint_account.key,
        //     initializer.key,
        //     None,
        //     0,
        // )?;

        // msg!("Before invoke mint_token_ix.");

        // // // Create mint token account
        // invoke(
        //     &mint_token_ix,
        //     &[
        //         mint_account.clone(),
        //         initializer.clone(),
        //         token_program.clone(),
        //     ],
        // )?;

        // msg!("After invoke mint_token_ix.");

        // Deserialize current event account data into a Rust struct.
        let mut event_info = Event::unpack_unchecked(&event_account.data.borrow())?;

        msg!("After unpack unchecked.");

        // Ensure account isn't already initialized.
        if event_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Set new values.
        event_info.is_initialized = true;
        event_info.initializer_pubkey = *initializer.key;
        event_info.tickets_issued = 0u64;
        event_info.max_tickets = max_tickets;
        event_info.name = name;
        event_info.mint_account = *mint_account.key;

        msg!("Event Info: {:?}", event_info);

        // Serialize back into account data to update the account.
        Event::pack(event_info, &mut event_account.data.borrow_mut())?;

        Ok(())
    }

    fn process_purchase_ticket(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
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

        // Ensure Event account is already initialized.
        if !event_info.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !(event_info.tickets_issued < event_info.max_tickets) {
            return Err(EventError::OutOfTickets.into());
        }

        // Increment tickets_issued;
        event_info.tickets_issued += 1;
        // Issue ticket

        msg!("Event Info: {:?}", event_info);
        // Serialize back into account data to update the account.
        Event::pack(event_info, &mut event_account.data.borrow_mut())?;

        Ok(())
    }
}
