```rust
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::PrintProgramError,
    pubkey::Pubkey,
};
use std::str::FromStr;

// Define the Message struct
#[derive(Default)]
struct Message {
    data: String,
}

// Define the program's entry point
entrypoint!(process_instruction);

// Process the instruction
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let message_account = next_account_info(accounts_iter)?;

    // Check if the message account is owned by the program
    if message_account.owner != program_id {
        msg!("Message account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the instruction data
    let instruction = instruction_data[0];

    match instruction {
        0 => create_message(program_id, message_account, instruction_data)?,
        1 => read_message(message_account)?,
        2 => update_message(program_id, message_account, instruction_data)?,
        3 => delete_message(program_id, message_account)?,
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidArgument);
        }
    }

    Ok(())
}

// Create a new message
fn create_message(
    program_id: &Pubkey,
    message_account: &AccountInfo,
    instruction_data: &[u8],
) -> ProgramResult {
    // Check if the message account is initialized
    if message_account.data_len() > 0 {
        msg!("Message account is already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Deserialize the message data
    let data = std::str::from_utf8(&instruction_data[1..]).unwrap();

    // Initialize the message account
    let message = Message { data: data.to_string() };
    let message_bytes = bincode::serialize(&message).unwrap();
    message_account.data.borrow_mut().copy_from_slice(&message_bytes);

    msg!("Message created successfully");
    Ok(())
}

// Read a message
fn read_message(message_account: &AccountInfo) -> ProgramResult {
    // Check if the message account is initialized
    if message_account.data_len() == 0 {
        msg!("Message account is not initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserialize the message data
    let message_bytes = message_account.data.borrow();
    let message: Message = bincode::deserialize(message_bytes).unwrap();

    msg!("Message data: {}", message.data);
    Ok(())
}

// Update a message
fn update_message(
    program_id: &Pubkey,
    message_account: &AccountInfo,
    instruction_data: &[u8],
) -> ProgramResult {
    // Check if the message account is owned by the program
    if message_account.owner != program_id {
        msg!("Message account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the message data
    let data = std::str::from_utf8(&instruction_data[1..]).unwrap();

    // Update the message account
    let message_bytes = message_account.data.borrow();
    let mut message: Message = bincode::deserialize(message_bytes).unwrap();
    message.data = data.to_string();
    let updated_message_bytes = bincode::serialize(&message).unwrap();
    message_account.data.borrow_mut().copy_from_slice(&updated_message_bytes);

    msg!("Message updated successfully");
    Ok(())
}

// Delete a message
fn delete_message(program_id: &Pubkey, message_account: &AccountInfo) -> ProgramResult {
    // Check if the message account is owned by the program
    if message_account.owner != program_id {
        msg!("Message account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Clear the message account data
    message_account.data.borrow_mut().fill(0);

    msg!("Message deleted successfully");
    Ok(())
}
```

###