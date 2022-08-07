use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::calculator::CalculatorInstructions;

pub mod calculator;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Calculator {
    pub value: u32,
}

entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo], // TransactionInstruction clientAccount!
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    // Get the account to interact/transact with
    // NOTE In this case, it's an account that stores data with a Calculator data structure
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct program id!");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Debug output:");
    msg!("Account ID: {}", account.key);
    msg!("Executable: {}", account.executable);
    msg!("Lamports: {:?}", account.lamports);
    msg!("Debug output complete.");

    // Program logic (e.g., do something with the account data)
    
    // NOTE The Borsh type comes with a prebuilt method try_from_slice.
    // This gets a String slice representation of the bytes that make up
    // our Calculator struct/schema. Meaning, it attempts to deserialize the
    // slice into a Calculator struct. 
    let mut calculator_account = Calculator::try_from_slice(&account.data.borrow())?;

    // Next, we can work with the data!
    // NOTE By adding/using the program's instruction data (CalculatorInstructions),
    // we now have more flexibility on how our program can manipulate the incoming
    // account's data. Instruction data makes our programs more robust.
    let calculator_instructions = CalculatorInstructions::try_from_slice(&instruction_data)?;
    msg!("Value BEFORE instruction function: {}", calculator_account.value);
    calculator_account.value = calculator_instructions.evaluate(calculator_account.value);

    // Finally, serialize it all back into Borsh type to store/save back into the account
    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Value AFTER instruction function: {}", calculator_account.value);
    

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
