use std::str::FromStr;

use alloy::{
    hex::FromHex,
    network::EthereumWallet,
    primitives::{Address, Bytes, U256, address, utils::format_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionInput, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::Result;
use reqwest::{Error, Response};
use serde::Deserialize;

use crate::{config, states::user_states::MemberProfile};
// use crate::xmtp::MemberProfile;

const CONTRACT_ADDRESS: Address = address!("0x525c2aba45f66987217323e8a05ea400c65d06dc");

#[derive(Deserialize, Debug)]
// This tells Serde to automatically map "Symbol" to "symbol", "Price" to "price", etc.
#[serde(rename_all = "PascalCase")]
struct EthResponse {
    symbol: String,
    name: String,
    address: String,
    blockchain: String,
    price: f64,
    // "PriceYesterday" maps seamlessly to price_yesterday
    price_yesterday: f64,
    // Note: For fields ending in an acronym like USD, PascalCase rules can sometimes get tricky.
    // If "VolumeYesterdayUSD" throws a missing field error, explicitly override it using the line below:
    #[serde(rename = "VolumeYesterdayUSD")]
    volume_yesterday_usd: f64,
    time: String,
    source: String,
    signature: String,
}

sol! {
    #[sol(rpc)] // Enables the generation of provider-compatible methods
    interface ICliqueProfile {
        // The individual profile structure returned by the single-call getter
        struct Profile {
            address walletAddress;
            string username;
            string bio;
            string avatarUrl;
        }

        // ==========================================
        // Getters (View functions) - NOW TAKE A USER ADDRESS
        // ==========================================
        function getWalletAddress(address user) external view returns (address);
        function getUsername(address user) external view returns (string memory);
        function getBio(address user) external view returns (string memory);
        function getAvatarUrl(address user) external view returns (string memory);

        // This is your single-call optimization lookup!
        // function getProfile(address user) external view returns ((address, string, string, string) memory);
        function getProfile(address user) external view returns ((address, string, string, string));
        function setUsername(string calldata new_username) external;
        function setBio(string calldata new_bio) external;
        function setAvatarUrl(string calldata new_avatar_url) external;

        // Combined profile initialization function
        function createProfile(
            string calldata new_username,
            string calldata new_bio,
            string calldata new_avatar_url
        ) external;
    }
}

pub async fn create_profile(username: String, bio: String, avatar: String) -> Result<()> {
    let signer: PrivateKeySigner = config::PRIVATE_KEY.parse()?;

    let address = signer.address();
    println!("ADDRESS FOR THIS TRANSACTION: {:?}", address);

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://localhost:8547")
        .await
        .expect("failed to create signer");

    // 3. Initialize the contract instance
    // let contract_address = address!("0xcEcba2F1DC234f70Dd89F2041029807F8D03A990");
    let contract = ICliqueProfile::new(CONTRACT_ADDRESS, provider);

    println!("Creating new profile with username: {:?}", username);
    // This broadcasts the transaction to the network
    // let tx_builder = contract.setUsername(username);
    let tx_builder = contract.createProfile(username, bio, avatar);
    let pending_tx = tx_builder
        .send()
        .await
        .expect("failed to build pending tx 1");

    println!("Transaction sent! Hash: {}", pending_tx.tx_hash());

    // 4. Wait for the transaction to be mined
    let receipt = pending_tx.get_receipt().await?;

    println!("Transaction confirmed in block: {:?}", receipt.block_number);

    Ok(())
}

pub async fn update_profile(
    username: Option<String>,
    description: Option<String>,
    pic: Option<String>,
) -> Result<()> {
    let signer: PrivateKeySigner = config::PRIVATE_KEY.parse()?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://localhost:8547")
        .await?;

    // 3. Initialize the contract instance
    // let contract_address = address!("0xcEcba2F1DC234f70Dd89F2041029807F8D03A990");
    let contract = ICliqueProfile::new(CONTRACT_ADDRESS, provider);

    println!(
        "Updating profile with username:  {:?}",
        username.clone().unwrap()
    );
    // This broadcasts the transaction to the network
    let tx_builder = contract.setUsername(username.unwrap());
    let pending_tx = tx_builder.send().await.expect("failed to build pending tx");
    println!("Transaction sent! Hash: {:?}", pending_tx.tx_hash());

    // 4. Wait for the transaction to be mined
    let receipt = pending_tx.get_receipt().await?;

    println!("Transaction confirmed in block: {:?}", receipt.block_number);

    Ok(())
}

pub async fn create_profile_gas_price(
    username: String,
    bio: String,
    avatar: String,
) -> Result<String> {
    let signer: PrivateKeySigner = config::PRIVATE_KEY.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://localhost:8547")
        .await
        .expect("failed to build provider");
    let contract = ICliqueProfile::new(CONTRACT_ADDRESS, &provider);

    let current_gas_price = provider
        .get_gas_price()
        .await
        .expect("failed to get gas price");
    println!("Current Gas Price: {} Gwei", current_gas_price);

    let test_string = String::from(
        r#"data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="100%" height="100%"><rect width="200" height="200" fill="%237F5AF0"/><circle cx="100" cy="85" r="35" fill="%23ffffff" opacity="0.3"/><text x="100" y="150" font-family="monospace" font-size="16" fill="%23ffffff" font-weight="bold" text-anchor="middle">cliqu3_dev</text></svg>"#,
    );
    let estimated_gas_units = contract
        .createProfile(username, bio, test_string)
        .estimate_gas()
        .await
        .expect("could not estimate gas");

    println!("Estimated Gas Units needed: {}", estimated_gas_units);

    // 4. Compute Total Price
    let total_cost_wei = current_gas_price * estimated_gas_units as u128;
    let total_cost_eth: String = format_units(total_cost_wei, "ether")?;
    println!("Total Transaction Cost: {} ETH", total_cost_eth);
    let eth_as_f64: f64 = total_cost_eth.parse().unwrap_or(0.0);
    let eth_price_usd = 2110.0; // Hardcoded spot estimate
    let url = "https://api.diadata.org/v1/assetQuotation/Ethereum/0x0000000000000000000000000000000000000000";
    let response: Response = reqwest::get(url).await.expect("failed to get eth price");
    let eth_json: EthResponse = response.json().await.expect("failed to get eth json");
    println!("Eth price found: {:?}", eth_json.price);
    println!(
        "Total Transaction Cost: ${:.5} USD",
        eth_as_f64 * eth_json.price
    );
    let gas_estimate = eth_as_f64 * eth_json.price;
    Ok(gas_estimate.to_string())
}

pub async fn test_contract() -> Result<()> {
    let signer: PrivateKeySigner = config::PRIVATE_KEY.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect("http://localhost:8547")
        .await
        .expect("failed to build provider");
    let contract = ICliqueProfile::new(CONTRACT_ADDRESS, &provider);

    let current_gas_price = provider
        .get_gas_price()
        .await
        .expect("failed to get gas price");
    println!("Current Gas Price: {} Gwei", current_gas_price);

    let test_string = String::from(
        r#"data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="100%" height="100%"><rect width="200" height="200" fill="%237F5AF0"/><circle cx="100" cy="85" r="35" fill="%23ffffff" opacity="0.3"/><text x="100" y="150" font-family="monospace" font-size="16" fill="%23ffffff" font-weight="bold" text-anchor="middle">cliqu3_dev</text></svg>"#,
    );
    let estimated_gas_units = contract
        .createProfile(
            "NewUserName".to_string(),
            "Some BIO... boring..".to_string(),
            test_string,
        )
        .estimate_gas()
        .await
        .expect("could not estimate gas");

    println!("Estimated Gas Units needed: {}", estimated_gas_units);

    // 4. Compute Total Price
    let total_cost_wei = current_gas_price * estimated_gas_units as u128;
    let total_cost_eth: String = format_units(total_cost_wei, "ether")?;
    println!("Total Transaction Cost: {} ETH", total_cost_eth);
    let eth_as_f64: f64 = total_cost_eth.parse().unwrap_or(0.0);
    let eth_price_usd = 2110.0; // Hardcoded spot estimate
    let url = "https://api.diadata.org/v1/assetQuotation/Ethereum/0x0000000000000000000000000000000000000000";
    let response: Response = reqwest::get(url).await.expect("failed to get eth price");
    let eth_json: EthResponse = response.json().await.expect("failed to get eth json");
    println!("Eth price found: {:?}", eth_json.price);
    println!(
        "Total Transaction Cost: ${:.5} USD",
        eth_as_f64 * eth_json.price
    );
    Ok(())
}

pub async fn get_profile(address_string: &str) -> Result<MemberProfile> {
    // let signer: PrivateKeySigner = config::PRIVATE_KEY.parse()?;
    let address_result = Address::from_str(address_string);

    let mut profile = MemberProfile {
        address: address_string.to_string(),
        name: "".to_string(),
        avatar: "".to_string(),
        description: "".to_string(),
    };

    let address: Address;
    match address_result {
        Ok(address_ok) => address = address_ok,
        Err(e) => return Ok(profile),
    }
    // let address = address!("0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e");
    //

    let provider = ProviderBuilder::new()
        .connect("http://localhost:8547")
        .await
        .expect("failed to build provider");

    let contract = ICliqueProfile::new(CONTRACT_ADDRESS, &provider);

    let (_, username, bio, avatar) = contract.getProfile(address).call().await?;

    // println!("address: {:?}", address);
    println!("username: {:?}", username);
    println!("bio: {:?}", bio);
    println!("avatar: {:?}", avatar);

    profile = MemberProfile {
        address: address_string.to_string(),
        name: username,
        avatar: avatar,
        description: bio,
    };

    Ok(profile)
}
