use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::str;

use crate::error::EventError;

#[derive(Debug)]
pub struct Event {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub tickets_issued: u64,
    pub max_tickets: u64,
    pub name: String,
    pub mint_account: Pubkey,
}

impl Sealed for Event {}

impl IsInitialized for Event {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Event {
    const LEN: usize = 113; // 1 + 32 + 8 + 8 + 32 + 32 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpack from slice");
        let src = array_ref![src, 0, Event::LEN];
        let (
            is_initialized,
            initializer_pubkey_array,
            tickets_issued_bytes,
            max_tickets_bytes,
            name_bytes,
            mint_account_bytes,
        ) = array_refs![src, 1, 32, 8, 8, 32, 32];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let initializer_pubkey = Pubkey::new_from_array(*initializer_pubkey_array);
        let tickets_issued = u64::from_le_bytes(*tickets_issued_bytes);
        let max_tickets = u64::from_le_bytes(*max_tickets_bytes);
        let name = match str::from_utf8(name_bytes) {
            Ok(s) => s.into(),
            Err(_) => return Err(EventError::InvalidEventName.into()),
        };
        let mint_account = Pubkey::new_from_array(*mint_account_bytes);

        Ok(Event {
            is_initialized,
            initializer_pubkey,
            tickets_issued,
            max_tickets,
            name,
            mint_account,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        msg!("pack into slice");
        let dst = array_mut_ref![dst, 0, Event::LEN];

        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            tickets_issued_dst,
            max_tickets_dst,
            name_dst,
            mint_account_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 32, 32];

        let Event {
            is_initialized,
            initializer_pubkey,
            tickets_issued,
            max_tickets,
            name,
            mint_account,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        *tickets_issued_dst = tickets_issued.to_le_bytes();
        *max_tickets_dst = max_tickets.to_le_bytes();
        name_dst.copy_from_slice(name[..32].as_bytes());
        mint_account_dst.copy_from_slice(mint_account.as_ref());
    }
}
