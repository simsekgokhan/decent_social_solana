use crate::{Error, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    pubkey::Pubkey, 
    signer::keypair::{read_keypair_file, Keypair}
};
use yaml_rust::YamlLoader;

/// This must be identical to the type defined in Solana program (smart contract)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserProfile {
    pub user_id: Pubkey,
    pub followers: u32,
    pub blocked_account: bool,
}

pub fn get_program_obj_size() -> Result<usize> {
    let encoded = 
        UserProfile {
            user_id: Pubkey::default(), 
            followers: 0, 
            blocked_account: false 
        }
        .try_to_vec()
        .map_err(Error::SerializationError)?;
    Ok(encoded.len())
    // E.g.
    // Ok(4 + (3 * 4)) // vec<u32> w/ 3 elements
    // Ok(3 * 4) // array[u32, 3] = 12 bytes
}

/// Add separator "_" for u64s, e.g. 3_834_260_000
pub fn add_separator(num: u64) -> String {
    num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
       .collect::<std::result::Result<Vec<&str>, _>>().unwrap().join("_")  
       // _ is separator
}

pub fn check_program_args() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 3 { return }
    panic!(
    "\nError: Wrong number of args.
       Usage:
       cargo r <unique_account_seed_string> <action>
       e.g. cargo r user1 t \n
    ",
    )
}

pub fn get_solana_config() -> Result<yaml_rust::Yaml> {
    let path = match home::home_dir() {
        Some(mut path) => {
            path.push(".config/solana/cli/config.yml");
            path
        }
        None => {
            return Err(Error::ConfigReadError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "failed to find home path and so cannot locate solana config",
            )));
        }
    };
    let config = std::fs::read_to_string(path).map_err(Error::ConfigReadError)?;
    let mut config = YamlLoader::load_from_str(&config)?;
    match config.len() {
        1 => Ok(config.remove(0)),
        l => Err(Error::InvalidConfig(format!(
            "expected one yaml document got ({})",
            l
        ))),
    }
}

pub fn get_rpc_url() -> Result<String> {
    let config = get_solana_config()?;
    match config["json_rpc_url"].as_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(Error::InvalidConfig(
            "missing `json_rpc_url` field".to_string(),
        )),
    }
}

pub fn get_user_keypair() -> Result<Keypair> {
    let config = get_solana_config()?;
    let path = match config["keypair_path"].as_str() {
        Some(s) => s,
        None => {
            return Err(Error::InvalidConfig(
                "missing `keypair_path` field".to_string(),
            ))
        }
    };
    read_keypair_file(path).map_err(|e| {
        Error::InvalidConfig(format!("failed to read keypair file ({}): ({})", path, e))
    })
}

pub fn seed_for_program_derived_account_creation() -> String {
    if cfg!(test) { return "user_test_99".to_string() }
    check_program_args();
    std::env::args().collect::<Vec<_>>()[1].clone() // e.g. "user1"
}

/// Return program derived account key for (user, program, seed)
pub fn pda_key(user: &Pubkey, program: &Pubkey) -> Result<Pubkey> {
    Ok(Pubkey::create_with_seed(
        user,
        &seed_for_program_derived_account_creation(),
        program,
    )?)
}

pub fn get_program_obj(data: &[u8]) -> Result<UserProfile> {
    let decoded = UserProfile::try_from_slice(data).map_err(
        Error::SerializationError
    )?;
    Ok(decoded)
}
