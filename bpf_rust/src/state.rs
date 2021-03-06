use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Event {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub max_tickets: u64,
}

impl Sealed for Event {}

impl IsInitialized for Event {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Event {
    const LEN: usize = 41; // 1 + 32 + 8 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Event::LEN];
        let (is_initialized, initializer_pubkey_array, max_tickets_bytes) =
            array_refs![src, 1, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let initializer_pubkey = Pubkey::new_from_array(*initializer_pubkey_array);
        let max_tickets = u64::from_le_bytes(*max_tickets_bytes);

        Ok(Event {
            is_initialized,
            initializer_pubkey,
            max_tickets,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Event::LEN];

        let (is_initialized_dst, initializer_pubkey_dst, max_tickets_dst) =
            mut_array_refs![dst, 1, 32, 8];

        let Event {
            is_initialized,
            initializer_pubkey,
            max_tickets,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        *max_tickets_dst = max_tickets.to_le_bytes();
    }
}
