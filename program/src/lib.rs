use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

// This is the state managed by this program. This type must match the 
// `UserProfile` type defined by the client/front_end
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserProfile {
    pub user_id: Pubkey,
    pub followers: u32,
    pub blocked_account: bool,
}

#[derive(Copy, Clone)]
enum ACTION {
    CreateNewProfile = 1, 
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let pda = next_account_info(accounts_iter)?;
    msg!("--- instruction_data: {:?}", instruction_data);
    msg!("--- pda: {}", pda.key);
    msg!("--- pda.UserProfile: {:?}", UserProfile::try_from_slice(&pda.data.borrow())?);
    // msg!("--- pda.data: {:?}", pda.data.borrow());
    msg!("--- accounts.len {}", accounts.len());

    // ACTION SELECTOR
    // todo: move to instructions.rs for production uses
    // todo: better use match but it will be two indentations and
    //       it is more readable with if due to cast to u8
    let fb = instruction_data[0]; // first byte
    if fb == ACTION::CreateNewProfile as u8 {
        msg!("--- instruction CreateNewProfile");
        let mut program_data = UserProfile::try_from_slice(&pda.data.borrow())?;
        program_data.user_id = *pda.key;
        program_data.blocked_account = false;
        program_data.followers = 100;
        program_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
        msg!("--- CreateNewProfile Success");
    }    else {
        todo!() 
    }

    Ok(())
}