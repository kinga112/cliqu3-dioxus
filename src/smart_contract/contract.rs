#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloy_primitives::Address;
use stylus_sdk::msg;
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress, StorageMap, StorageString};

// 1. Define a separate layout block for an individual User Profile
sol_storage! {
    pub struct UserProfile {
        address wallet_address;
        string username;
        string bio;
        string avatar_url;
    }
}

// 2. The main contract storage now maps user addresses to their personal profiles
sol_storage! {
    #[entrypoint]
    pub struct Cliqu3Profile {
        // Maps: Account Owner Address => UserProfile Data State
        mapping(address => UserProfile) profiles;
    }
}

#[public]
impl Cliqu3Profile {
    // ==========================================
    // GETTERS (Open to ALL users via address args)
    // ==========================================

    /// Fetches the profile data for ANY user on the platform by address
    pub fn get_profile(&self, user: Address) -> Result<(Address, String, String, String), Vec<u8>> {
        // Look up the specific profile nested inside the map state
        let profile = self.profiles.getter(user);

        Ok((
            profile.wallet_address.get(),
            profile.username.get_string(),
            profile.bio.get_string(),
            profile.avatar_url.get_string(),
        ))
    }

    pub fn get_username(&self, user: Address) -> Result<String, Vec<u8>> {
        Ok(self.profiles.getter(user).username.get_string())
    }

    pub fn get_bio(&self, user: Address) -> Result<String, Vec<u8>> {
        Ok(self.profiles.getter(user).bio.get_string())
    }

    pub fn get_avatar_url(&self, user: Address) -> Result<String, Vec<u8>> {
        Ok(self.profiles.getter(user).avatar_url.get_string())
    }

    // ==========================================
    // SETTERS (Protected implicitly using msg::sender)
    // ==========================================

    /// Initializes a brand new profile tied directly to the caller's signing key
    pub fn create_profile(
        &mut self,
        new_username: String,
        new_bio: String,
        new_avatar_url: String,
    ) -> Result<(), Vec<u8>> {
        // Cryptographically verify who is signing this execution request
        let caller = msg::sender();

        // Target ONLY their specific mapping slot
        let mut profile = self.profiles.setter(caller);

        profile.wallet_address.set(caller);
        profile.username.set_str(new_username);
        profile.bio.set_str(new_bio);
        profile.avatar_url.set_str(new_avatar_url);

        Ok(())
    }

    pub fn set_username(&mut self, new_username: String) -> Result<(), Vec<u8>> {
        let caller = msg::sender();
        self.profiles.setter(caller).username.set_str(new_username);
        Ok(())
    }

    pub fn set_bio(&mut self, new_bio: String) -> Result<(), Vec<u8>> {
        let caller = msg::sender();
        self.profiles.setter(caller).bio.set_str(new_bio);
        Ok(())
    }

    pub fn set_avatar_url(&mut self, new_avatar_url: String) -> Result<(), Vec<u8>> {
        let caller = msg::sender();
        self.profiles
            .setter(caller)
            .avatar_url
            .set_str(new_avatar_url);
        Ok(())
    }
}
