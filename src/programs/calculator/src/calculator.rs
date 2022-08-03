use borsh::{BorshDeserialize, BorshSerialize};


// NOTE By adding/using the program's instruction data (CalculatorInstructions),
// we now have more flexibility on how our program can manipulate the incoming
// account's data. Instruction data makes our programs more robust.
// NOTE This struct represents TWO pieces of instruction data we need
// NOTE In the Web3 Client, this struct is a Buffer in instruction_data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstructions {
    // NOTE For simplicity, we're using numbers to represent the
    // different operations.
    // TODO Could improve on this by creating an Enum of the various
    // operations available.
    operation: u32, // add, sub, mult, etc.
    operation_value: u32, // by what value
}

impl CalculatorInstructions {
    pub fn evaluate(self, value: u32) -> u32 {
        // Modify the incoming value
        match &self.operation {
            1 => value + &self.operation_value,
            2 => value - &self.operation_value,
            3 => value * &self.operation_value,
            _ => value * 0,
        } 
    }
}
