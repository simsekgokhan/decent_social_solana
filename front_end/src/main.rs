use front_end::client::{
    create_pda, create_new_user_profile, get_program_obj, get_program, 
    print_program_info
};
use front_end::utils::seed_for_program_derived_account_creation;

fn main() {
    // 1. Connect to Solana devnet
    let connection = front_end::client::establish_connection();
    let user = front_end::utils::get_user().unwrap();

    // todo: make this fn
    // 2. Create account (if needed) for program to write its data 
    println!("\n>> Create account for program to read/write its data...");
    // todo: check arg count
    let args = std::env::args().collect::<Vec<_>>();
    let program_keypair = &args[1];
    let program = get_program(program_keypair, &connection).unwrap();
    let res = create_pda(&user, &program, &connection).unwrap();
    println!("--- result : {:?}", res);

    // 3. Print some info
    print_program_info(&user, &connection, &program);

    // 4. Create new user profile
    println!("\n>> Creating new user profile...");
    let res = create_new_user_profile(&user, &program, &connection);
    println!("--- result : {:?}", res);

    // 5. Get chain data
    println!("\n>> Retreving chain data...");
    let program_obj = get_program_obj(&user, &program, &connection).unwrap();
    let seed = seed_for_program_derived_account_creation();
    println!("\nProgram Object for account seed '{}':\n{:#?}", seed, program_obj);

    println!("\nEnd\n");
}
