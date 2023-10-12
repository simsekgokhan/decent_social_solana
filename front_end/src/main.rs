use front_end::client::{
    create_pda, create_new_user_profile_tx, establish_connection, 
    get_program_obj, get_program_keypair, print_program_info
};
use front_end::utils::{
    check_program_args, get_user_keypair, seed_for_program_derived_account_creation
};

/// Summary:
/// This frontend program will create a UserProfile object on Solana
/// devnet chain and will read the created data from chain.
/// Hint: See README.md for dev. env. and account setup at root dir.
/// 
/// Usage: 
/// cargo r <unique_account_seed_string>
/// e.g. cargo r user1
fn main() {
    check_program_args();
    
    // Connect to Solana devnet
    let connection = establish_connection();
    let user_keypair = get_user_keypair().unwrap();

    // Create account (if needed) for program to write its data 
    println!("\n>> Create account for program to read/write its data...");    
    let program_keypair = get_program_keypair(&connection).unwrap();
    let result = create_pda(&user_keypair, &program_keypair, &connection).unwrap();
    println!("--- result : {:?}", result);

    // Print some info
    print_program_info(&user_keypair, &connection, &program_keypair);

    // Create new user profile
    println!("\n>> Creating new user profile...");
    let result = create_new_user_profile_tx(&user_keypair, &program_keypair, &connection);
    println!("--- result : {:?}", result);

    // Get chain data
    println!("\n>> Retreving chain data...");
    let program_obj = get_program_obj(&user_keypair, &program_keypair, &connection).unwrap();
    let seed = seed_for_program_derived_account_creation();
    println!("\nProgram Object for account seed '{}':\n{:#?}", seed, program_obj);

    println!("\nEnd\n");
}
