use borsh::{BorshDeserialize, BorshSerialize};
use crate::instruction::init_new_profile;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

mod instruction;

entrypoint!(process_instruction);

// This is the state managed by this program. This type must match the 
// `UserProfile` type defined by the client/front_end
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UserProfile {
    pub user_id: Pubkey,
    pub followers: u32,
    pub blocked_account: bool,
}

#[derive(Copy, Clone)]
enum Action {
    CreateNewProfile         = 1, // Creates a new user profile on-chain
    TransferSolFromPdaToUser = 2, 
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let pda = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;

    // ACTION SELECTOR
    // todo: better to use match here but it will be two indentations and
    //       it is more readable with if due to cast to u8
    let fb = instruction_data[0]; // first byte
    if fb == Action::CreateNewProfile as u8 {
        msg!("--- Executing instruction CreateNewProfile ...");
        let mut user_profile = UserProfile::try_from_slice(&pda.data.borrow())?;
        init_new_profile(&mut user_profile, *pda.key);
        user_profile.serialize(&mut &mut pda.data.borrow_mut()[..])?;
        msg!("--- CreateNewProfile Success");
    } 
    else if fb == Action::TransferSolFromPdaToUser as u8{
        msg!("--- Executing instruction TransferSolFromPdaToUser ...");
        mspl::system::transfer_sol(&pda, &user, 10);
        msg!("--- TransferSolFromPdaToUser Success");
    }
    else {
      todo!() 
    }

    Ok(())
}