use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    /// calculated value
    pub result: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstructions {
    operation: u8,
    first_number: u8,
    second_number: u8,
}

impl CalculatorInstructions {
    pub fn evaluate(&self) -> u8 {
        match self.operation {
            0 => self.first_number + self.second_number,
            1 => self.first_number - self.second_number,
            _ => 0,
        }
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!("Instruction data: {:?}", _instruction_data);

    let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;
    let calculator_instructions =
        CalculatorInstructions::try_from_slice(&_instruction_data).unwrap();

    calculator_account.result = calculator_instructions.evaluate() as u32;
    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Calc result is {}", calculator_account.result);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let sum_instruction_data: Vec<u8> = vec![0, 5, 8];
        let subtract_instruction_data: Vec<u8> = vec![1, 8, 5];
        let accounts = vec![account];

        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .result,
            0
        );
        process_instruction(&program_id, &accounts, &sum_instruction_data).unwrap();
        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .result,
            13
        );
        process_instruction(&program_id, &accounts, &subtract_instruction_data).unwrap();
        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .result,
            3
        );
    }
}
