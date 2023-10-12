use crate::client::{
    create_pda, create_new_user_profile_tx, establish_connection, 
    get_program_obj, get_program_keypair, print_program_info
};
use crate::utils::{    
    get_user_keypair, pda_key, seed_for_program_derived_account_creation
};
use solana_sdk::signature::Signer;

#[test]
fn test_new_user_profile_with_dummy_values() {
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

    let pda = pda_key(&user_keypair.pubkey(), &program_keypair.pubkey()).unwrap();
    assert_eq!(program_obj.user_id, pda);
    assert_eq!(program_obj.followers, 100);
    assert_eq!(program_obj.blocked_account, false);
    assert_eq!(seed, "user_test_99");
}