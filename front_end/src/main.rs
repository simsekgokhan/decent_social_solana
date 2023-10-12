use front_end::client::{
    create_pda, create_new_user_profile, get_program_obj, get_program, 
    print_program_info
};

fn main() {
    // 1. Connect to Solana devnet
    let connection = front_end::client::establish_connection();
    let user = front_end::utils::get_user().unwrap();

    // todo: make this fn
    // 2. Create account (if needed) for program to write its data 
    println!("\n2. Create account for program to read/write its data...");
    // todo: check arg count
    let args = std::env::args().collect::<Vec<_>>();
    let program_keypair = &args[1];
    let program = get_program(program_keypair, &connection).unwrap();
    let res = create_pda(&user, &program, &connection).unwrap();
    println!("--- res : {:?}", res);

    // 3. Print some info
    print_program_info(&user, &connection, &program);

    // 4. Create new user profile
    println!("\n2. Creating new user profile...");
    let res = create_new_user_profile(&user, &program, &connection);
    println!("--- res : {:?}", res);

    let program_obj = get_program_obj(&user, &program, &connection).unwrap();
    println!("\nprogram_obj:\n{:#?}", program_obj);

    println!("\nEnd\n");
}
