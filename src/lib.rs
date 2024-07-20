use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Entrypoint tanımı
entrypoint!(process_instruction);

// Hesap yapısı
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub counter: u32,
}

// Talimat enum'ı
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(u32),
}

// Talimatları işleme fonksiyonu
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstructions::try_from_slice(instruction_data)?;
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId.into());
    }

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(value) => {
            counter_account.counter = counter_account.counter.checked_add(value).unwrap_or(u32::MAX);
        }
        CounterInstructions::Decrement(value) => {
            counter_account.counter = counter_account.counter.checked_sub(value).unwrap_or(0);
        }
        CounterInstructions::Update(value) => {
            counter_account.counter = value;
        }
    }

    counter_account.serialize(&mut *account.data.borrow_mut())?;
    Ok(())
}

// Hata işleme
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CounterError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}