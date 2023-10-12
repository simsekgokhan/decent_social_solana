use crate::{Error, Result};
use crate::utils::{pp, pda_key, seed_for_program_derived_account_creation};
use solana_client::rpc_client::RpcClient;
use solana_program::native_token::lamports_to_sol;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    signature::Signer,
    signer::keypair::{read_keypair_file, Keypair},
    transaction::Transaction
};

pub fn establish_connection() -> RpcClient {
    let connection = RpcClient::new_with_commitment(
        crate::utils::get_rpc_url().unwrap(),
        CommitmentConfig::confirmed(),
    );
    println!(
        "\n>> Connected to remote solana node running version ({}).\n",
        connection.get_version().unwrap()
    );
    connection
}

pub fn print_program_info(user: &Keypair, connection: &RpcClient, program: &Keypair) {
    println!("\n>> Info");
    let user_balance = get_user_balance(&user, &connection).unwrap();
    println!("User   : {:?}", user.pubkey());
    println!("Balance: {} Sol ({} lamports)", 
        lamports_to_sol(user_balance), pp(user_balance)
    );    
    println!("Program: {:?}", program.pubkey());
    let pda = pda_key(&user.pubkey(), &program.pubkey()).unwrap();
    println!("PDA    : {:?}", pda);
    println!("  (aka Program's data account to read/write)");
    println!("  (aka Derived addr for a given user and program combination)");
    println!("PDA seed: {}\n", seed_for_program_derived_account_creation());
}

pub fn get_user_balance(user: &Keypair, connection: &RpcClient) -> Result<u64> {
    Ok(connection.get_balance(&user.pubkey())?)
}

pub fn get_program(keypair_path: &str, connection: &RpcClient) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).map_err(|e| {
        Error::InvalidConfig(format!(
            "failed to read program keypair file ({}): ({})",
            keypair_path, e
        ))
    })?;

    let program_info = connection.get_account(&program_keypair.pubkey())?;
    if !program_info.executable {
        return Err(Error::InvalidConfig(format!(
            "program with keypair ({}) is not executable",
            keypair_path
        )));
    }

    Ok(program_keypair)
}

pub fn create_pda(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
) -> Result<()> {
    let program_derived_account = pda_key(&user.pubkey(), &program.pubkey())?;
    let program_obj_size = crate::utils::get_program_obj_size().unwrap();
    println!("--- Program's object size: {} bytes", program_obj_size);
    let lamport_requirement = connection.get_minimum_balance_for_rent_exemption(
        program_obj_size
    )?;
    println!("--- min_balance_for_rent_exemption: {}", pp(lamport_requirement));

    let mut success = false;
    if let Err(_) = connection.get_account(&program_derived_account) {
        println!("... creating program derived account");
        let instruction = solana_sdk::system_instruction::create_account_with_seed(
            &user.pubkey(),
            &program_derived_account,
            &user.pubkey(),
            &crate::utils::seed_for_program_derived_account_creation(),
            lamport_requirement,
            program_obj_size as u64,
            &program.pubkey(),
        );
        let message = Message::new(&[instruction], Some(&user.pubkey()));
        let transaction =
            Transaction::new(&[user], message, connection.get_latest_blockhash()?);

        let signature = connection.send_and_confirm_transaction(&transaction)?;
        success = true;
        println!("Signature: {}", signature);
    }

    if !success { println!("... not created, account may already exist "); }

    Ok(())
}


pub fn get_program_obj(
    user: &Keypair, program: &Keypair, connection: &RpcClient
) -> Result<crate::utils::UserProfile> {
    let account_key = 
        pda_key(&user.pubkey(), &program.pubkey())?;
    let account = connection.get_account(&account_key)?;
    // println!("--- program derived account: {:?}", &account.data);
    Ok(crate::utils::get_program_obj(&account.data)?)
}

pub fn create_new_user_profile(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
) -> Result<()> {
    let pda = pda_key(&user.pubkey(), &program.pubkey())?;
    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[1],
        vec![
            AccountMeta::new(pda, false),
        ],
    );
    let message = Message::new(&[instruction], Some(&user.pubkey()));
    let transaction = Transaction::new(
        &[user], message, connection.get_latest_blockhash()?
    );

    let _sig = connection.send_and_confirm_transaction(&transaction)?;
    // println!("sig: {}", sig);

    Ok(())
}
