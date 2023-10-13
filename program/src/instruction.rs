use solana_program::pubkey::Pubkey;
use crate::UserProfile;
#[allow(unused_imports)]
use std::str::FromStr; // this is used, disabling false warning

// todo: return result
pub fn init_new_profile(user_profile: &mut UserProfile, pda_key: Pubkey)  {
    user_profile.user_id = pda_key;
    user_profile.blocked_account = false;
    user_profile.followers = 100;
}

#[cfg(test)]
mod profile_unit_tests {
    use super::*;

    #[test]
    fn create_new_profile_with_default_values() {
        let user_profile = UserProfile::default();
        let pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        assert_eq!(user_profile.user_id, pubkey);
        assert_eq!(user_profile.blocked_account, false);
        assert_eq!(user_profile.followers, 0);
    }

    #[test]
    fn init_new_profile_() {
        let mut user_profile = UserProfile::default();
        let pubkey = Pubkey::from_str("2ULuUe9z1fYKv5GC9UrFTztCQpnBsU8M3SjCoJVZh2GA").unwrap();
        init_new_profile(&mut user_profile, pubkey);
        assert_eq!(user_profile.user_id, pubkey);
        assert_eq!(user_profile.blocked_account, false);
        assert_eq!(user_profile.followers, 100);
    }
}